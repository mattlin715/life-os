import { invoke, isTauri } from "@tauri-apps/api/core";
import { createJsonProvider } from "./sharedJsonProvider";
function parse(text: string): Record<string, unknown> { const value = JSON.parse(text.trim().replace(/^```(?:json)?/i, "").replace(/```$/i, "").trim()); if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("AI response was not a JSON object."); return value; }
async function request(instructions: string, input: unknown) { if (!isTauri()) throw new Error("Gemini provider is only enabled in Tauri."); return parse(await invoke<string>("generate_gemini_response", { instructions, input: JSON.stringify(input, null, 2) })); }
export const geminiProvider = createJsonProvider(request);
