// Thin wrapper over grammy's Bot class. One class = one responsibility:
// own the Bot instance, expose a narrow surface used by tool handlers.

import { Bot, InputFile } from "grammy";
import type { ContactRecord, GroupRecord } from "./types.js";

export interface TelegramClientConfig {
  token: string;
}

export class TelegramClient {
  private readonly bot: Bot;
  private readonly contactsCache: Map<number, ContactRecord> = new Map();
  private readonly groupsCache: Map<number, GroupRecord> = new Map();

  constructor(cfg: TelegramClientConfig) {
    if (!cfg.token) throw new Error("TELEGRAM_BOT_TOKEN is required");
    this.bot = new Bot(cfg.token);
  }

  async status(): Promise<{ username: string; id: number }> {
    const me = await this.bot.api.getMe();
    return { username: me.username, id: me.id };
  }

  async chatInfo(chat: string | number): Promise<{ id: number; title: string; type: string }> {
    const info = await this.bot.api.getChat(chat);
    const title = "title" in info && info.title ? info.title :
      "first_name" in info && info.first_name ? info.first_name : String(info.id);
    return { id: info.id, title, type: info.type };
  }

  async sendText(chat: string | number, text: string): Promise<number> {
    const msg = await this.bot.api.sendMessage(chat, text);
    return msg.message_id;
  }

  async sendDocument(chat: string | number, filePath: string, caption?: string): Promise<number> {
    const msg = await this.bot.api.sendDocument(chat, new InputFile(filePath), caption !== undefined ? { caption } : {});
    return msg.message_id;
  }

  async sendPhoto(chat: string | number, filePath: string, caption?: string): Promise<number> {
    const msg = await this.bot.api.sendPhoto(chat, new InputFile(filePath), caption !== undefined ? { caption } : {});
    return msg.message_id;
  }

  async sendVideo(chat: string | number, filePath: string, caption?: string): Promise<number> {
    const msg = await this.bot.api.sendVideo(chat, new InputFile(filePath), caption !== undefined ? { caption } : {});
    return msg.message_id;
  }

  async sendVoice(chat: string | number, filePath: string, caption?: string): Promise<number> {
    const msg = await this.bot.api.sendVoice(chat, new InputFile(filePath), caption !== undefined ? { caption } : {});
    return msg.message_id;
  }

  listGroups(): GroupRecord[] {
    return Array.from(this.groupsCache.values());
  }

  listContacts(): ContactRecord[] {
    return Array.from(this.contactsCache.values());
  }

  // Test helpers to seed cache; kept internal via underscore prefix.
  _seedContact(c: ContactRecord): void {
    this.contactsCache.set(c.userId, c);
  }

  _seedGroup(g: GroupRecord): void {
    this.groupsCache.set(g.chatId, g);
  }
}
