// Gmail API tool I/O types. Types live in their own file so tests can
// exercise schemas without importing googleapis.

import { z } from "zod";

export const ListUnreadArgs = z.object({
  max: z.number().int().positive().max(500).default(20),
});
export type ListUnreadArgs = z.infer<typeof ListUnreadArgs>;

export const GetMessageArgs = z.object({
  id: z.string().min(1),
});
export type GetMessageArgs = z.infer<typeof GetMessageArgs>;

export const SearchArgs = z.object({
  query: z.string().min(1),
  max: z.number().int().positive().max(500).default(20),
});
export type SearchArgs = z.infer<typeof SearchArgs>;

export const LabelArgs = z.object({
  id: z.string().min(1),
  label: z.string().min(1),
});
export type LabelArgs = z.infer<typeof LabelArgs>;

export const ModifyOnlyArgs = z.object({
  id: z.string().min(1),
});
export type ModifyOnlyArgs = z.infer<typeof ModifyOnlyArgs>;

export interface MessageSummary {
  id: string;
  threadId?: string | undefined;
  subject?: string | undefined;
  from?: string | undefined;
  snippet?: string | undefined;
  date?: string | undefined;
}
