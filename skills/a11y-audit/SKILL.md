---
name: a11y-audit
description: Use when auditing accessibility — WCAG 2.2 AA compliance, contrast checks, keyboard navigation, screen reader support, prefers-reduced-motion. Triggers on "accessibility", "a11y", "wcag", "screen reader", "contrast check".
arguments:
  - name: command
    description: "Command: scan, fix, contrast, checklist, report"
    required: false
  - name: target
    description: URL or file path to audit
    required: false
---

# Accessibility Audit — WCAG 2.2 AA

## Legal Context

- **EAA (EU):** In force since June 2025. Penalties: up to 100K EUR / 4% revenue
- **ADA (US):** References WCAG 2.2 AA
- **Standard:** WCAG 2.2 AA is minimum for any commercial site targeting US/EU

## Top 10 Violations

1. Missing alt text on images
2. Low contrast text (4.5:1 normal, 3:1 large text)
3. Keyboard traps in menus
4. Missing form labels
5. Skipped heading levels
6. No skip links
7. Non-semantic HTML (`<div>` instead of `<nav>`, `<main>`)
8. Missing video captions
9. Invisible focus styles
10. Touch targets <24x24px (WCAG 2.2 new)

**Automated tools catch only 30-40%.** Manual audit required.

## Automated Testing

```bash
# Lighthouse CLI
npx lighthouse <url> --output=json --only-categories=accessibility
# axe-core
npx @axe-core/cli <url> --tags wcag2a,wcag2aa,wcag22aa
```

Playwright integration:
```javascript
import AxeBuilder from '@axe-core/playwright';
const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag22aa']).analyze();
expect(results.violations).toEqual([]);
```

## CSS Media Queries

### prefers-reduced-motion
```css
@media (prefers-reduced-motion: reduce) {
  .animated { animation: fade-in 0.2s ease; transition: opacity 0.2s ease; }
  .parallax { transform: none !important; }
  .scroll-animation { animation: none; }
}
```
Replace motion (slide, scale) with non-motion (fade, opacity). Keep transitions <200ms.

### prefers-color-scheme / prefers-contrast / forced-colors
Always support dark mode, high contrast, and Windows forced colors.

## WCAG 2.2 New Criteria

- **2.5.8:** Touch targets min 24x24 CSS px
- **2.4.11/12:** Focus not obscured by sticky elements
- **3.3.7:** No redundant entry (don't re-ask info)
- **3.3.8:** No cognitive tests for auth (allow password managers)
- **2.5.7:** Dragging has non-drag alternative

## Semantic HTML Reference

```html
<a href="#main" class="skip-link">Skip to content</a>
<header><nav aria-label="Main">...</nav></header>
<main id="main">
  <section aria-labelledby="heading"><h2 id="heading">...</h2></section>
</main>
<footer>...</footer>
```

## Manual Checklist

- [ ] Keyboard-only: Tab through entire page, verify focus order
- [ ] Skip link visible on focus
- [ ] All interactive elements: visible focus indicator
- [ ] Heading hierarchy: one h1, no skipped levels
- [ ] All images: meaningful alt OR aria-hidden="true" (decorative)
- [ ] Color contrast: 4.5:1 normal, 3:1 large (18px+ bold or 24px+)
- [ ] Forms: visible labels, errors linked with aria-describedby
- [ ] ARIA landmarks: header, nav, main, footer
- [ ] Touch targets: 24x24px minimum
- [ ] Animations: respect prefers-reduced-motion
- [ ] Dark mode: all elements visible and contrasted
- [ ] Video: captions present, controls accessible
- [ ] `lang` attribute on `<html>`
- [ ] Link text descriptive (not "click here")
- [ ] Errors announced to screen readers (aria-live)
