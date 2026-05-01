---
name: ui-component
description: Use when building a UI component — API design, variants, accessibility, animations, tests
arguments:
  - name: component
    description: Component name and description
    required: true
  - name: framework
    description: "Framework: react, next, astro, svelte, vue (auto-detect if omitted)"
    required: false
---

# UI Component Workflow

## Step 1: Research
- Check if component exists in project already (Glob/Grep)
- Check existing component library for similar components
- Review design system tokens if available
- Identify the component's role and variations needed

## Step 2: API Design (Props First)
Define before implementing:
```
interface ComponentProps {
  // Required props
  // Optional props with defaults
  // Event handlers
  // Composition slots (children, render props)
  // Style overrides (className, style)
}
```
- Keep API minimal — only props that are actually needed
- Use discriminated unions for variant props
- Sensible defaults for all optional props

## Step 3: Implementation
- Follow project's component patterns exactly
- Compose from existing primitives when possible
- Variants via props, not separate components

### Accessibility
- Semantic HTML elements
- ARIA attributes where needed
- Keyboard navigation (Tab, Enter, Escape, Arrow keys)
- Focus management and visible focus styles
- Screen reader announcements for dynamic content
- Color contrast WCAG AA (4.5:1 text, 3:1 large/UI)

### Animations
- Use CSS transitions/animations over JS when possible
- Respect `prefers-reduced-motion`
- Consistent timing from design system tokens
- Enter/exit animations for conditional rendering

## Step 4: Tests
- Render test (mounts without error)
- Props test (each variant renders correctly)
- Interaction test (click, hover, keyboard)
- Accessibility test (axe-core or similar)

## Step 5: Examples
- Default usage
- All variants
- With different content lengths
- Responsive behavior
- Dark mode
