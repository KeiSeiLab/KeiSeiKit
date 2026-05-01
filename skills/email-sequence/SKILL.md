---
name: email-sequence
description: Use when creating email sequences — welcome, onboarding, nurture funnels with HTML templates
arguments:
  - name: type
    description: "Sequence type: welcome, onboarding, nurture, reactivation, launch"
    required: true
  - name: product
    description: Product/service name
    required: true
  - name: emails
    description: Number of emails in sequence (default 5)
    required: false
---

# Email Sequence Workflow

## Step 1: Define Sequence Strategy
Based on type:
- **Welcome (3-5 emails):** introduce → value → quick win → explore features → CTA
- **Onboarding (5-7 emails):** setup → first action → core feature → advanced → success story → upgrade
- **Nurture (4-6 emails):** value content → case study → problem/solution → social proof → offer
- **Reactivation (3 emails):** "we miss you" → what's new → last chance offer
- **Launch (4-5 emails):** teaser → announcement → features deep-dive → social proof → final CTA

## Step 2: Plan Each Email
For each email define:
- **Subject line** (2 options: curiosity + benefit-driven)
- **Preview text** (<90 chars)
- **Send timing** (day + time relative to trigger)
- **Goal** (one action per email)
- **Content outline** (3-5 bullet points)

## Step 3: Write Copy
- Subject: <50 chars, no spam triggers (FREE, !!!, ALL CAPS)
- Preview: complements subject, creates curiosity
- Body structure:
  1. Personal opener (1-2 sentences)
  2. Value / story (2-3 paragraphs)
  3. Single CTA button
  4. P.S. line (optional, high-impact)
- Tone: conversational, direct, benefit-focused
- Length: 150-300 words per email

## Step 4: HTML Templates
- Generate responsive HTML email template
- Inline CSS (email client compatibility)
- Max width: 600px
- Single column layout
- CTA button: min 44x44px touch target, contrasting color
- Footer: unsubscribe link, company info
- Test: renders in Gmail, Apple Mail, Outlook

## Step 5: Output
- Complete email sequence document
- HTML templates for each email
- Subject line A/B test variants
- Recommended sending schedule
