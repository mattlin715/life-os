import { describe, expect, it, vi } from "vitest";
import { requestLifeOsClose } from "./closeLifeOs";

describe("Life OS close control", () => {
  it("awaits the desktop close command", async () => {
    const closeDesktop = vi.fn(async () => undefined);
    const closeBrowser = vi.fn();

    await requestLifeOsClose({ isDesktop: true, closeDesktop, closeBrowser });

    expect(closeDesktop).toHaveBeenCalledOnce();
    expect(closeBrowser).not.toHaveBeenCalled();
  });

  it("surfaces a rejected desktop close command", async () => {
    const failure = new Error("window_close_refused");

    await expect(requestLifeOsClose({
      isDesktop: true,
      closeDesktop: vi.fn(async () => { throw failure; }),
      closeBrowser: vi.fn(),
    })).rejects.toBe(failure);
  });

  it("uses the browser close fallback outside Tauri", async () => {
    const closeDesktop = vi.fn(async () => undefined);
    const closeBrowser = vi.fn();

    await requestLifeOsClose({ isDesktop: false, closeDesktop, closeBrowser });

    expect(closeDesktop).not.toHaveBeenCalled();
    expect(closeBrowser).toHaveBeenCalledOnce();
  });
});
