# AI-Init Integration Plan - Post V2 Testing

**Date:** 2026-05-04
**Based on:** V2 tooling comparison test results
**Goal:** Make ai-init lean, effective, and token-efficient by default

---

## Executive Summary

**Test Results:** AI-init added **13,000+ token overhead** without improving quality or speed.

**Root Cause:** Too much documentation in generated files:
- CLAUDE.md: 2,500 tokens (too verbose)
- TOOLS.md: 4,400 tokens (most expensive, least valuable)
- Total overhead: ~13,000 tokens

**Solution:** Make ai-init minimal by default, remove bloat, keep project-specific value.

---

## Core Philosophy Change

### ❌ OLD Approach (Current)
"Generate comprehensive documentation files explaining everything"

**Result:**
- 13,000 token overhead
- Agent spent tokens reading docs instead of code
- Tools could be in MCP server anyway
- Verbose files not fully utilized

### ✅ NEW Approach (Proposed)
"Generate minimal project-specific context only"

**Result:**
- ~1,500-2,000 token overhead (85% reduction)
- Agent gets straight to work
- Tools discoverable via MCP (optional)
- Files are actually useful

---

## Changes to Implement

### 1. Make Minimal Mode the DEFAULT ⭐

**Current:** `ai-init .` generates full verbose docs
**New:** `ai-init .` generates minimal lean docs

**Flags:**
- `ai-init .` → Minimal mode (new default)
- `ai-init . --verbose` → Full documentation mode (old default)
- `ai-init . --mcp` → MCP-aware mode (skip TOOLS.md entirely)

**Why:** Test showed verbose docs waste tokens without adding value.

---

### 2. Redesign CLAUDE.md Template (Critical)

**Current Size:** 83 lines, ~2,500 tokens
**Target Size:** 15-20 lines, ~500 tokens

**NEW CLAUDE.md.tera Template:**

```markdown
# {{ project_name }}

{{ description }}

**Languages:** {{ languages | join(sep=", ") }} | **Type:** {{ project_type }} | **Created:** {{ created_date }}

---

## Development Principles

Read `.ai/CONVENTIONS.md` before coding:
- Test-first development
- Keep it simple
- Files < 300 lines
- Use libraries over custom code

## Architecture

See `.ai/ARCHITECTURE.md` for system design and component structure.

{% if tools_available > 0 %}## Available Tools

{% if tool_code_index %}**code-index** - Symbol lookup, dependency queries
{% endif %}{% if tool_code_summarizer %}**code-summarizer** - Hotspots, complexity metrics
{% endif %}{% if tool_context_query %}**context-query** - Structure-aware search
{% endif %}{% if tool_context_packer %}**context-packer** - Smart context assembly
{% endif %}
Run tools on-demand as needed.
{% endif %}
```

**Token Savings:** 2,500 → 500 tokens (**-2,000 tokens, 80% reduction**)

**What's Removed:**
- ❌ Verbose explanations of what tools do (redundant)
- ❌ "For AI Agents" sections (agents know what to do)
- ❌ Development workflow steps (in CONVENTIONS.md)
- ❌ Multiple references to "read TOOLS.md first" (unnecessary pressure)

**What's Kept:**
- ✅ Project metadata
- ✅ Pointers to CONVENTIONS.md and ARCHITECTURE.md
- ✅ Tool availability (brief list)
- ✅ Core principles

---

### 3. REMOVE TOOLS.md by Default ⭐⭐⭐

**Current:** Always generated, 4,400 tokens
**New:** Only generate if `--verbose` flag used

**Reasoning:**
1. **If MCP available:** Tools auto-discovered, TOOLS.md redundant
2. **If no MCP:** Agent can run `--help` on tools to learn usage
3. **Agent in V2 test:** Used code-index successfully despite verbose docs
4. **Token cost:** 4,400 tokens is the single biggest overhead

**Behavior:**
- `ai-init .` → No TOOLS.md (saves 4,400 tokens)
- `ai-init . --verbose` → Generate full TOOLS.md
- `ai-init . --mcp` → No TOOLS.md (MCP provides tools)

**Token Savings:** -4,400 tokens (most impactful change)

---

### 4. Simplify TOOLS.md Template (If Generated)

**Current Size:** 103 lines, ~4,400 tokens
**Target Size:** 30 lines, ~800 tokens

**NEW TOOLS.md.tera Template (Minimal):**

```markdown
# Available Tools

{{ tools_available }}/{{ tools | length }} installed

{% for tool in tools %}
## {{ tool.name }}
{% if tool.installed %}✅ `{{ tool.path }}`{% else %}❌ Not installed{% endif %}

{{ tool.description }}

```bash
{{ tool.usage }}
```
{% endfor %}

Run `<tool> --help` for full documentation.
```

**Token Savings:** 4,400 → 800 tokens (**-3,600 tokens when --verbose used**)

**What's Removed:**
- ❌ Extended usage examples (use --help instead)
- ❌ Best practices section (agents know this)
- ❌ "AI Agent Note" sections (condescending)
- ❌ Detailed output descriptions (discoverable via usage)

**What's Kept:**
- ✅ Tool name and status
- ✅ Basic description
- ✅ Primary usage command
- ✅ Reference to --help for details

---

### 5. Keep CONVENTIONS.md and ARCHITECTURE.md (Unchanged)

**Why:** These provide **project-specific context** that tools can't discover:
- Coding standards specific to this project
- Test-first requirements
- File organization patterns
- System architecture and component relationships

**Token Cost:**
- CONVENTIONS.md: ~4,600 tokens
- ARCHITECTURE.md: ~1,200 tokens
- Total: ~5,800 tokens

**Value:** HIGH (these are unique to each project)

**Action:** No changes needed - these templates are good.

---

### 6. Add MCP-Aware Mode

**New Flag:** `ai-init . --mcp`

**Behavior:**
- Skip TOOLS.md entirely (tools in MCP)
- Minimal CLAUDE.md (500 tokens)
- Keep CONVENTIONS.md + ARCHITECTURE.md
- Add note to CLAUDE.md: "Tools available via MCP server"

**Token Overhead:** ~6,300 tokens (vs 13,000 currently)

**When to Use:** User has MCP server set up with code-intelligence tools

---

## Summary of Changes

### File Generation Changes

| File | Current | Minimal (Default) | Verbose (--verbose) | MCP (--mcp) |
|------|---------|-------------------|---------------------|-------------|
| CLAUDE.md | 2,500 tokens | **500 tokens** ✅ | 2,500 tokens | **500 tokens** ✅ |
| TOOLS.md | 4,400 tokens | **Not generated** ✅ | 800 tokens | **Not generated** ✅ |
| CONVENTIONS.md | 4,600 tokens | 4,600 tokens | 4,600 tokens | 4,600 tokens |
| ARCHITECTURE.md | 1,200 tokens | 1,200 tokens | 1,200 tokens | 1,200 tokens |
| **TOTAL OVERHEAD** | **12,700** | **6,300** (50% cut) | **9,100** (28% cut) | **6,300** (50% cut) |

### Token Savings

- **Minimal mode (new default):** -6,400 tokens (50% reduction)
- **Verbose mode (optional):** -3,600 tokens (28% reduction)
- **MCP mode (optional):** -6,400 tokens (50% reduction)

---

## Implementation Checklist

### Phase 1: Minimal Mode (Priority 1) ⚡

**Effort:** 4-6 hours

- [ ] Create new `CLAUDE.md.minimal.tera` template (~500 tokens)
- [ ] Create new `TOOLS.md.minimal.tera` template (~800 tokens)
- [ ] Update `src/cli.rs`: Add `--verbose` flag
- [ ] Update `src/generator.rs`: Default to minimal templates
- [ ] Update `src/generator.rs`: Skip TOOLS.md unless `--verbose`
- [ ] Update README.md: Document new default behavior
- [ ] Update tests to expect minimal output
- [ ] Tag release: `v0.3.0-minimal-by-default`

**Result:** Default `ai-init .` now generates 50% less token overhead.

---

### Phase 2: MCP Mode (Priority 2) 🔌

**Effort:** 2-3 hours

- [ ] Update `src/cli.rs`: Add `--mcp` flag
- [ ] Update `src/generator.rs`: Skip TOOLS.md when `--mcp` flag
- [ ] Update CLAUDE.md template: Add MCP-aware variant
- [ ] Add note to README.md about MCP mode
- [ ] Update tests for MCP mode
- [ ] Tag release: `v0.4.0-mcp-support`

**Result:** Users with MCP can skip TOOLS.md entirely.

---

### Phase 3: Template Cleanup (Priority 3) 🧹

**Effort:** 2-3 hours

- [ ] Rename current templates to `.verbose.tera` suffix
- [ ] Set `.minimal.tera` as defaults
- [ ] Remove duplicate/redundant template text
- [ ] Add comments to templates explaining token budgets
- [ ] Update template variables for clarity
- [ ] Tag release: `v0.5.0-template-cleanup`

**Result:** Codebase cleaner, templates maintainable.

---

## Breaking Changes Assessment

### For Users

**BREAKING:** Default behavior changes from verbose to minimal.

**Migration:**
- Users wanting old behavior: `ai-init . --verbose`
- Update documentation/guides referencing ai-init
- Bump major version to 1.0.0 (semantic versioning)

**Recommendation:** Call this `v1.0.0` - it's a mature, opinionated default.

---

### For Agents

**NOT BREAKING:** Agents adapt to available files.

**What Agents See:**
- Minimal mode: Less to read, faster startup
- Still get project conventions (the valuable part)
- Tools discoverable via execution or MCP

**Benefit:** Agents are more efficient with minimal mode.

---

## Post-Integration Testing

### Test Suite Updates

**Add Tests:**
1. Minimal mode generates correct files
2. Verbose mode generates full docs
3. MCP mode skips TOOLS.md
4. Token counts within budget:
   - Minimal CLAUDE.md < 600 tokens
   - Minimal TOOLS.md < 900 tokens (if generated)
   - Total overhead < 7,000 tokens

**Regression Tests:**
- All existing tests pass with `--verbose` flag
- Generated files are valid markdown
- Template variables render correctly

---

### V3 Comparison Test

**After implementation, run V3 test:**

1. `ai-init . --mcp` on same actix-web repo
2. Run same 10-task test
3. Measure token usage
4. Compare:
   - V2 Agent A (old ai-init): 138,156 tokens
   - V2 Agent B (no tools): 49,857 tokens
   - V3 Agent C (new minimal): **Target: 55,000-65,000 tokens**

**Success Criteria:**
- Token usage within 20% of baseline (Agent B)
- All tasks completed
- Quality maintained

---

## Features to REMOVE

### 1. Pre-Generation of Context Files ❌

**Current:** Some users expect HOTSPOTS.md pre-generated
**Remove:** Don't pre-generate any context files
**Reason:** Wasteful - files loaded but not fully used

### 2. Verbose "AI Agent Note" Sections ❌

**Current:** Templates have "AI Agent Note:" sections
**Remove:** All of these
**Reason:** Condescending, agents don't need coaching

### 3. Redundant References ❌

**Current:** "Read TOOLS.md FIRST", "Read CONVENTIONS.md BEFORE coding"
**Remove:** The pressure language
**Replace:** Simple "See .ai/CONVENTIONS.md for coding standards"
**Reason:** Agents make their own decisions about reading order

### 4. Best Practices Sections ❌

**Current:** TOOLS.md has "Best Practices for AI Agents"
**Remove:** Entire section
**Reason:** Agents learn by doing, not by reading meta-advice

### 5. Installation Status Verbosity ❌

**Current:** "If any tools show as 'NOT INSTALLED', they can be built from specs..."
**Remove:** Long explanations
**Replace:** Simple ✅/❌ indicator
**Reason:** If not installed, agent asks user or moves on

---

## Features to KEEP ✅

### 1. Project-Specific Context

**Keep:** CONVENTIONS.md, ARCHITECTURE.md
**Why:** This is the core value - can't be discovered elsewhere

### 2. Tera Templates

**Keep:** Template system for customization
**Why:** Users can still override if needed

### 3. Tool Detection

**Keep:** Scanning for installed tools
**Why:** Useful to know what's available

### 4. Git Integration

**Keep:** Optional git init and .gitignore handling
**Why:** Convenience feature that doesn't add token overhead

### 5. Update Mode

**Keep:** `--update` flag for existing repos
**Why:** Valuable for refreshing AI files

---

## Configuration File Changes

### ~/.config/ai-init/config.toml

**Add New Defaults:**

```toml
[generation]
# Default template mode (minimal, verbose)
default_mode = "minimal"

# Generate TOOLS.md by default
generate_tools = false

# MCP server configured (skip TOOLS.md)
mcp_enabled = false

[tokens]
# Target token budgets
claude_md_budget = 600
tools_md_budget = 1000
total_overhead_budget = 7000
```

**Validation:**
- Warn if generated files exceed budgets
- Suggest --verbose if users want full docs

---

## Documentation Updates

### README.md Changes

**Add Section:**

```markdown
## Token Efficiency

ai-init is optimized for minimal token overhead:

- **Minimal mode (default):** ~6,300 token overhead
- **Verbose mode:** ~9,100 token overhead
- **MCP mode:** ~6,300 token overhead (no TOOLS.md)

### Modes

```bash
# Minimal (default) - lean and efficient
ai-init my-project

# Verbose - full documentation
ai-init my-project --verbose

# MCP-aware - assumes tools in MCP server
ai-init my-project --mcp
```

**Update Examples:**
- Show minimal mode as primary example
- Add note about verbose mode for teams
- Explain MCP mode benefits

---

## Migration Guide for Users

### For v0.2.x Users Upgrading to v1.0.0

**Breaking Change:** Default output is now minimal.

**To Get Old Behavior:**
```bash
# Add --verbose flag
ai-init . --verbose
```

**Recommended:** Try minimal mode first, it's faster and more efficient.

**Benefits of Minimal:**
- 50% less token overhead
- Faster agent startup
- Cleaner, more focused files
- Same quality results

**When to Use Verbose:**
- Team documentation needs
- Learning what tools do
- Comprehensive reference

---

## Success Metrics

### Token Efficiency

**Target:**
- Default mode < 7,000 token overhead
- Verbose mode < 10,000 token overhead
- 50%+ reduction from v0.2.x

**Measure:**
- Count tokens in generated files
- V3 comparison test
- User feedback on token usage

### Adoption

**Target:**
- 80%+ users stick with default minimal mode
- Low requests for "bring back verbose default"

**Measure:**
- GitHub issues
- User surveys
- Usage telemetry (if implemented)

### Quality

**Target:**
- Agent task completion rate unchanged
- Output quality maintained
- Faster time to useful work

**Measure:**
- V3 test results
- User reported issues
- Agent success rates

---

## Timeline

### Week 1: Core Implementation
- Day 1-2: New minimal templates
- Day 3-4: CLI flags and generator logic
- Day 5: Testing and bug fixes

### Week 2: Polish and Release
- Day 1-2: Documentation updates
- Day 3: V3 comparison test
- Day 4-5: Release v1.0.0

**Total Effort:** ~10 days (1 developer, full-time)

---

## FAQ

### Q: Why remove TOOLS.md by default?

**A:** It's 4,400 tokens and doesn't help agents discover tools better. Agents can:
1. Use MCP-provided tools (if available)
2. Run `tool --help` to learn usage
3. Experiment and learn by doing

The verbose documentation doesn't improve outcomes but costs tokens.

---

### Q: Won't agents miss important tool documentation?

**A:** No. The V2 test showed:
- Agent A (with verbose TOOLS.md): Used tools successfully
- Agent B (without TOOLS.md): Completed tasks successfully
- No evidence verbose docs improved tool usage

Agents are smart enough to figure it out.

---

### Q: What if users want comprehensive docs?

**A:** Use `--verbose` flag to get full documentation mode.

---

### Q: Is this compatible with MCP?

**A:** Yes! Use `--mcp` flag when you have MCP server set up. It skips TOOLS.md entirely since tools are in MCP.

---

### Q: Will this break existing projects?

**A:** No. Existing projects keep their files. This only affects NEW projects created with `ai-init`.

To update existing projects: `ai-init . --update` (uses minimal mode by default)

---

## Conclusion

**Core Change:** Make ai-init minimal by default.

**Why:** Test proved verbose documentation wastes tokens without improving outcomes.

**Impact:** 50% token reduction, cleaner files, same quality.

**Migration:** Simple - add `--verbose` if you want old behavior.

**Timeline:** 2 weeks to implement and release v1.0.0.

---

## Next Steps

1. ✅ Review this integration plan
2. ⏭️ Approve/modify changes
3. ⏭️ Implement Phase 1 (minimal mode)
4. ⏭️ Test with V3 comparison
5. ⏭️ Release v1.0.0
6. ⏭️ Gather user feedback
7. ⏭️ Iterate based on real-world usage

---

**Prepared by:** Claude Sonnet 4.5
**Based on:** V2 Tooling Comparison Test Results
**Status:** Ready for implementation
