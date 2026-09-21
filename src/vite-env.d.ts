/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_LIFE_OS_DESKTOP_SCHEMA_V5?: string;
  readonly VITE_LIFE_OS_FOUNDER_SCHEMA_V5?: string;
  readonly VITE_LIFE_OS_ANDROID_FEASIBILITY_M0?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
