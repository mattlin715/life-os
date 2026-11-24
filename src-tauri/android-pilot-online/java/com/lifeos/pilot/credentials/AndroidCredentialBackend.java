package com.lifeos.pilot.credentials;

import android.content.Context;
import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;
import android.util.AtomicFile;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.nio.charset.StandardCharsets;
import java.security.KeyStore;
import java.util.Arrays;
import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.spec.GCMParameterSpec;

/** No ordinary preference, journal database, export, backup or device-transfer credential copy. */
public final class AndroidCredentialBackend implements CredentialStore.Backend {
    private static final String ALIAS = "lifeos-model-pilot-api-wrap-v1";
    private static final byte[] MAGIC = new byte[] { 'L', 'O', 'S', 'K', 1 };
    private final AtomicFile file;
    private final byte[] aad;

    public AndroidCredentialBackend(Context context) {
        if (!"com.lifeos.review.pilot.model".equals(context.getPackageName())) throw new SecurityException("credential_unavailable");
        file = new AtomicFile(new File(context.getNoBackupFilesDir(), "model-api-key-v1.enc"));
        aad = (context.getPackageName() + ":" + ALIAS).getBytes(StandardCharsets.US_ASCII);
    }

    private KeyStore keyStore() throws Exception {
        KeyStore store = KeyStore.getInstance("AndroidKeyStore");
        store.load(null);
        return store;
    }

    private SecretKey wrappingKey(boolean create) throws Exception {
        KeyStore store = keyStore();
        if (store.containsAlias(ALIAS)) return (SecretKey) store.getKey(ALIAS, null);
        if (!create) throw new Exception("credential_unavailable");
        KeyGenerator generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore");
        generator.init(new KeyGenParameterSpec.Builder(ALIAS, KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT)
                .setKeySize(256).setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                .setRandomizedEncryptionRequired(true).build());
        return generator.generateKey();
    }

    @Override public byte[] read() throws Exception {
        if (!file.getBaseFile().exists() && !new File(file.getBaseFile().getPath() + ".bak").exists()) return null;
        try (FileInputStream input = file.openRead(); ByteArrayOutputStream out = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[512];
            int size;
            while ((size = input.read(buffer)) != -1) {
                if (out.size() + size > 4096) throw new Exception("credential_unavailable");
                out.write(buffer, 0, size);
            }
            return out.toByteArray();
        }
    }

    @Override public void write(byte[] ciphertext) throws Exception {
        FileOutputStream output = null;
        try {
            output = file.startWrite();
            output.write(ciphertext);
            file.finishWrite(output);
        } catch (Exception error) {
            if (output != null) file.failWrite(output);
            throw new Exception("credential_unavailable");
        }
    }

    @Override public byte[] seal(byte[] plaintext) throws Exception {
        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.ENCRYPT_MODE, wrappingKey(true));
        cipher.updateAAD(aad);
        byte[] iv = cipher.getIV();
        if (iv.length != 12) throw new Exception("credential_unavailable");
        byte[] encrypted = cipher.doFinal(plaintext);
        byte[] result = new byte[MAGIC.length + iv.length + encrypted.length];
        System.arraycopy(MAGIC, 0, result, 0, MAGIC.length);
        System.arraycopy(iv, 0, result, MAGIC.length, iv.length);
        System.arraycopy(encrypted, 0, result, MAGIC.length + iv.length, encrypted.length);
        return result;
    }

    @Override public byte[] open(byte[] blob) throws Exception {
        if (blob.length < MAGIC.length + 12 + 16 || !Arrays.equals(MAGIC, Arrays.copyOf(blob, MAGIC.length)))
            throw new Exception("credential_unavailable");
        Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
        cipher.init(Cipher.DECRYPT_MODE, wrappingKey(false), new GCMParameterSpec(128, blob, MAGIC.length, 12));
        cipher.updateAAD(aad);
        return cipher.doFinal(blob, MAGIC.length + 12, blob.length - MAGIC.length - 12);
    }

    @Override public void destroy() throws Exception {
        KeyStore store = keyStore();
        if (store.containsAlias(ALIAS)) store.deleteEntry(ALIAS);
    }
}
