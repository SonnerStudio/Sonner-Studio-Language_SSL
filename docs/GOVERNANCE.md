# SSL Governance Model

## Project Leadership

### Core Team

The Core Team consists of project maintainers with commit access and decision-making authority.

**Current Members:**
- SonnerStudio (@SonnerStudio) - Project Lead

**Responsibilities:**
- Strategic direction and roadmap
- RFC approval/rejection
- Release management
- Mentoring contributors
- Code review for critical changes

### Adding Core Members

New core team members can be nominated by existing members based on:
- Sustained high-quality contributions (>10 merged PRs)
- Domain expertise
- Community involvement
- Requires unanimous approval from existing core team

## Decision-Making Process

### Standard Changes

**Examples:** Bug fixes, documentation, examples, non-breaking features

**Process:**
1. Open Pull Request
2. Automated CI checks pass
3. One core team member approval
4. Merge

### RFC-Required Changes

**Examples:** New language features, breaking changes, major architecture changes

**Process:**
1. Author creates RFC document (docs/rfcs/NNNN-feature.md)
2. Open Pull Request for RFC
3. Community discussion (minimum 2 weeks)
4. Core team review
5. Decision: Accept, Reject, or Postpone
6. If accepted: Implementation PR follows

### Release Decisions

- Minor releases (0.x.0): Core team consensus
- Patch releases (0.x.y): Any maintainer can release
- Major releases (1.0+): Unanimous core team approval

## RFC Process

### When to Write an RFC

- Adding/removing language syntax
- Breaking API changes
- Significant performance trade-offs
- Major architectural decisions

### RFC Lifecycle

1. **Draft**: Copy template, fill sections, open PR
2. **Discussion**: 14-day minimum comment period
3. **Final Comment Period (FCP)**: 7 days after team signals decision
4. **Resolution**: 
   - **Accepted**: Merged into docs/rfcs/
   - **Rejected**: Closed with rationale
   - **Postponed**: Labeled for future consideration

### RFC Template

See [docs/rfcs/0000-template.md](docs/rfcs/0000-template.md)

Required sections:
- Summary
- Motivation
- Detailed Design
- Drawbacks
- Alternatives
- Unresolved Questions

## Contribution Rights

### Anyone Can:
- Report bugs
- Suggest features (via issues)
- Participate in discussions
- Submit PRs (after review)

### Contributors:
- Recognized for merged PRs
- Listed in CONTRIBUTORS.md
- Invited to community events

### Core Team:
- Commit access
- RFC voting rights
- Release permissions
- Moderation powers

## Conflict Resolution

1. **Technical Disagreements**: RFC process for structured discussion
2. **Code Review Disputes**: Escalate to core team
3. **Code of Conduct Violations**: Lead maintainer final authority

## Transparency

- All decisions documented in GitHub (Issues, PRs, RFCs)
- Roadmap published in ROADMAP.md
- Release notes for every version
- Monthly status updates (when active)

## Changes to Governance

This document itself requires an RFC to modify.

---

**Version:** 1.0  
**Last Updated:** 2025-11-28  
**Contact:** hbcomputer@freenet.de
