---
name: form-builder
description: Use when building forms — multi-step wizards, Zod validation, anti-spam (Turnstile), serverless backends, file upload, progressive enhancement. Triggers on "form", "contact form", "wizard", "form validation", "turnstile".
arguments:
  - name: command
    description: "Command: create, validate, backend, spam, analytics, audit"
    required: false
  - name: type
    description: "Form type: contact, multi-step, file-upload, survey"
    required: false
---

# Form Construction & Submission

Progressive enhancement by default. Forms MUST work without JavaScript.

## Architecture

```
User → Client Validation (Zod) → Submit
  ↓ (JS disabled: standard POST)
Server Action/Worker → Server Validation (same Zod schema)
  ↓
Anti-spam (Turnstile) → Process → Email (Resend) / Webhook / D1
```

## Validation: Zod v4 + react-hook-form v7

**Single schema shared between client and server (SSoT):**

```typescript
// schemas/contact.ts
import { z } from 'zod';
export const contactSchema = z.object({
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  company: z.string().optional(),
  message: z.string().min(10).max(5000),
  budget: z.enum(['<5k', '5k-15k', '15k-50k', '50k+']),
});
export type ContactFormData = z.infer<typeof contactSchema>;
```

**Client form:**
```tsx
const { register, handleSubmit, formState: { errors, isSubmitting } } = useForm<ContactFormData>({
  resolver: zodResolver(contactSchema),
});
// method="POST" action="/api/contact" — works without JS
// noValidate — use Zod, not browser
// aria-describedby + aria-invalid + role="alert" for a11y
```

**WARNING:** react-hook-form v8 in beta with breaking changes. Stick to v7.

## Multi-Step Wizard

- Schema per step, merged for final validation
- `sessionStorage` for persistence across refreshes
- Progress indicator, back navigation, summary before submit
- Validate current step before "Next"

## Anti-Spam

### Cloudflare Turnstile (DEFAULT — free, unlimited, privacy-friendly)
```html
<div class="cf-turnstile" data-sitekey="YOUR_KEY"></div>
```
Server: verify via `challenges.cloudflare.com/turnstile/v0/siteverify`

### Honeypot (always layer with Turnstile)
```html
<div style="position:absolute;left:-9999px" aria-hidden="true">
  <input type="text" name="website" tabindex="-1" autocomplete="off" />
</div>
```

### Rate Limiting
5 submissions/IP/hour via Cloudflare KV.

## Backends

| Backend | Best For |
|---------|----------|
| CF Worker + Resend | Email notifications (DEFAULT) |
| Webhook | Slack/Discord/Zapier/n8n |
| D1 | Persistent storage + analytics |
| R2 presigned URL | File uploads (>5MB use multipart) |

## Form Types

| Type | Fields | Anti-Spam | Backend |
|------|--------|-----------|---------|
| Contact | name, email, message, budget? | Turnstile + honeypot | Resend + webhook |
| Multi-step | per-step schemas | Turnstile on final | D1 + Resend |
| File upload | name, email, file(s) | Turnstile + rate limit | R2 presigned |
| Survey | rating, category, text | honeypot + rate limit | D1 |

## Audit Checklist

- [ ] All fields: visible `<label>`, aria-describedby for errors
- [ ] Works without JS (method + action set)
- [ ] Server validation matches client (same Zod schema)
- [ ] Anti-spam: honeypot minimum, Turnstile preferred
- [ ] Rate limiting on endpoint
- [ ] File uploads: presigned URLs (not Worker proxy)
- [ ] Input types match data (email, tel, url)
- [ ] Autocomplete attributes set
- [ ] Submit disabled during submission
- [ ] Success/error announced to screen readers
- [ ] Mobile: 44x44px touch targets, appropriate keyboards
