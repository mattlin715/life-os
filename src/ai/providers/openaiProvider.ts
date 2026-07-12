import { invoke, isTauri } from "@tauri-apps/api/core";
import { createJsonProvider } from "./sharedJsonProvider";
export interface AiRuntimeStatus { provider: "openai" | "gemini" | "mock"; model?: string; reason?: string; }
function parse(text: string): Record<string, unknown> { const value = JSON.parse(text.trim().replace(/^```(?:json)?/i, "").replace(/```$/i, "").trim()); if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("AI response was not a JSON object."); return value; }
async function request(instructions: string, input: unknown) { if (!isTauri()) throw new Error("OpenAI provider is only enabled in Tauri."); return parse(await invoke<string>("generate_openai_response", { instructions, input: JSON.stringify(input, null, 2) })); }
export async function getAiRuntimeStatus(): Promise<AiRuntimeStatus> { return isTauri() ? invoke<AiRuntimeStatus>("get_ai_runtime_status") : { provider: "mock", reason: "Real AI is only enabled in Tauri." }; }
export const openaiProvider = createJsonProvider(request);
