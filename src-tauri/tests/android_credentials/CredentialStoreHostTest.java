package com.lifeos.pilot.credentials;

import java.nio.charset.StandardCharsets;
import java.security.SecureRandom;
import java.util.Arrays;
import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.spec.GCMParameterSpec;

/** Synthetic injected backend only: does not access Android, a real keystore, or user files. */
public final class CredentialStoreHostTest {
    private static int checks;
    private static final String KEY = "synthetic-host-key-never-transmit";
    private static void check(boolean value) { if (!value) throw new AssertionError(); checks++; }
    private interface Action { void run() throws Exception; }
    private static void rejects(Action action) throws Exception {
        boolean rejected = false;
        try { action.run(); } catch (Exception expected) { rejected = true; }
        check(rejected);
    }

    private static final class Backend implements CredentialStore.Backend {
        byte[] stored;
        SecretKey wrapping;
        int writes;
        boolean writeFails, encryptFails, destroyFails;
        public byte[] read() { return stored == null ? null : stored.clone(); }
        public void write(byte[] blob) throws Exception {
            writes++;
            if (writeFails) throw new Exception("synthetic failure");
            stored = blob.clone();
        }
        public void destroy() throws Exception {
            if (destroyFails) throw new Exception("synthetic failure");
            wrapping = null;
        }
        public byte[] seal(byte[] plain) throws Exception {
            if (encryptFails) throw new Exception("synthetic failure");
            if (wrapping == null) { KeyGenerator generator = KeyGenerator.getInstance("AES"); generator.init(256); wrapping = generator.generateKey(); }
            byte[] iv = new byte[12]; new SecureRandom().nextBytes(iv);
            Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
            cipher.init(Cipher.ENCRYPT_MODE, wrapping, new GCMParameterSpec(128, iv));
            byte[] encrypted = cipher.doFinal(plain), blob = Arrays.copyOf(iv, iv.length + encrypted.length);
            System.arraycopy(encrypted, 0, blob, iv.length, encrypted.length);
            return blob;
        }
        public byte[] open(byte[] blob) throws Exception {
            if (wrapping == null) throw new Exception("synthetic failure");
            Cipher cipher = Cipher.getInstance("AES/GCM/NoPadding");
            cipher.init(Cipher.DECRYPT_MODE, wrapping, new GCMParameterSpec(128, blob, 0, 12));
            return cipher.doFinal(blob, 12, blob.length - 12);
        }
    }

    public static void main(String[] args) throws Exception {
        Backend backend = new Backend(); CredentialStore store = new CredentialStore(backend);
        check(store.load() == null && backend.writes == 0 && backend.wrapping == null);
        store.replace(KEY);
        check(!new String(backend.stored, StandardCharsets.US_ASCII).contains(KEY));
        check(KEY.equals(new CredentialStore(backend).load()));
        byte[] first = backend.stored.clone();
        store.replace(KEY); check(!Arrays.equals(first, backend.stored));
        int writes = backend.writes;
        for (String key : new String[] { "", "too-short", KEY + "\n", KEY + "日", "a".repeat(2049) }) rejects(() -> store.replace(key));
        check(backend.writes == writes);
        byte[] good = backend.stored.clone();
        backend.stored[backend.stored.length - 1] ^= 1;
        rejects(store::load); check(backend.writes == writes); // no automatic recovery or plaintext fallback
        backend.stored = good.clone(); backend.wrapping = null;
        rejects(store::load); check(backend.writes == writes && backend.wrapping == null);
        store.replace(KEY);
        backend.encryptFails = true;
        good = backend.stored.clone(); rejects(() -> store.replace(KEY + "_new"));
        check(Arrays.equals(good, backend.stored)); backend.encryptFails = false;
        backend.writeFails = true;
        rejects(() -> store.replace(KEY + "_new")); check(KEY.equals(store.load()));
        rejects(() -> store.replace(null)); check(KEY.equals(store.load()));
        backend.writeFails = false;
        backend.destroyFails = true;
        rejects(() -> store.replace(null)); check(store.load() == null); // disabled before destroy: no resurrection
        check(backend.stored.length == 0 && backend.wrapping != null);
        backend.destroyFails = false; store.replace(null);
        check(new CredentialStore(backend).load() == null && backend.wrapping == null);
        store.replace(KEY); store.replace(null); // opting back into session-only forgets retained key
        check(new CredentialStore(backend).load() == null);
        backend.stored = new byte[4097]; rejects(store::load);
        System.out.println("CREDENTIAL_LIFECYCLE_HOST_CHECKS=" + checks + "; PASS (injected backend, not Android Keystore)");
    }
}
