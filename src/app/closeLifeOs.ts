export interface LifeOsClosePort {
  readonly isDesktop: boolean;
  readonly closeDesktop: () => Promise<void>;
  readonly closeBrowser: () => void;
}

export async function requestLifeOsClose(port: LifeOsClosePort): Promise<void> {
  if (port.isDesktop) {
    await port.closeDesktop();
    return;
  }

  port.closeBrowser();
}
