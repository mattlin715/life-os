package com.lifeos.pilot.credentials;

import java.nio.charset.StandardCharsets;
import java.util.Arrays;

/** Native-only secret lifecycle; the backend owns authenticated encryption and atomic storage. */
public final class CredentialStore {
    public interface Backend {
        byte[] read() throws Exception;
        void write(byte[] ciphertext) throws Exception;
        byte[] seal(byte[] plaintext) throws Exception;
        byte[] open(byte[] ciphertext) throws Exception;
        void destroy() throws Exception;
    }

    private final Backend backend;

    public CredentialStore(Backend backend) { this.backend = backend; }

    public static boolean validKey(String key) {
        if (key == null || key.length() < 16 || key.length() > 2048) return false;
        for (int i = 0; i < key.length(); i++) {
            char c = key.charAt(i);
            if (!((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
                    || c == '-' || c == '_' || c == '.')) return false;
        }
        return true;
    }

    public synchronized String load() throws Exception {
        byte[] blob = backend.read();
        if (blob == null || blob.length == 0) return null;
        if (blob.length > 4096) throw new Exception("credential_unavailable");
        byte[] plain = null;
        try {
            plain = backend.open(blob);
            if (plain == null || plain.length < 16 || plain.length > 2048) throw new Exception("credential_unavailable");
            // Strict ASCII: a replacement character must never become a different credential.
            for (byte b : plain) if (b < 0) throw new Exception("credential_unavailable");
            String key = new String(plain, StandardCharsets.US_ASCII);
            if (!validKey(key)) throw new Exception("credential_unavailable");
            return key;
        } finally {
            Arrays.fill(blob, (byte) 0);
            if (plain != null) Arrays.fill(plain, (byte) 0);
        }
    }

    /** Null means session-only/forget. An empty atomic marker disables restore before key removal. */
    public synchronized void replace(String key) throws Exception {
        if (key == null) {
            backend.write(new byte[0]);
            backend.destroy();
            return;
        }
        if (!validKey(key)) throw new Exception("invalid_key");
        byte[] plain = key.getBytes(StandardCharsets.US_ASCII);
        byte[] blob = null;
        try {
            blob = backend.seal(plain);
            if (blob == null || blob.length <= plain.length || blob.length > 4096) throw new Exception("credential_unavailable");
            backend.write(blob);
        } finally {
            Arrays.fill(plain, (byte) 0);
            if (blob != null) Arrays.fill(blob, (byte) 0);
        }
    }
}
