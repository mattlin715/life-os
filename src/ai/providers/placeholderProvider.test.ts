import { describe, expect, it } from "vitest";
import { placeholderProvider } from "./placeholderProvider";
import { createContextPacket } from "../harness/contextPacket";
import { evidence, experience } from "../../test/fixtures";
describe("mock provider copy", () => {
  it("uses safe English quotation and clean multilingual output", async () => {
    const english = createContextPacket({ currentExperience: experience("en", "A meeting stayed with me."), evidence: [evidence("e", "en")], locale: "en", requestedTask: "pattern", provider: "mock" }).packet;
    const [note] = await placeholderProvider.suggestPatternNotes(english);
    expect(note.text).toContain('"The meeting stayed on my mind."');
    expect(note.text).not.toContain("?".repeat(4));
    expect(note.text).not.toContain(String.fromCharCode(0xfffd));
  });
});
