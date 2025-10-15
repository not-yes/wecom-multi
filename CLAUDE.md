# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

企业微信多开工具 (WeCom Multi-Open) - A cross-platform tool to run multiple instances of WeChat Work (企业微信) simultaneously.

**Core Principle**: Enables multi-instance by closing system-level Mutex objects (Windows) or using native multi-instance commands (macOS), without modifying the target application.

## Build & Run Commands

### Development
```bash
# Run CLI version (default 3 instances)
cargo run

# Run with specific instance count
cargo run -- 5

# Run with debug logging
RUST_LOG=debug cargo run
```

### Building
```bash
# Build release (optimized)
cargo build --release

# Windows quick build
build.bat

# Output location
target/release/wecom-multi-open     # Unix
target/release/wecom-multi-open.exe # Windows
```

### Testing
```bash
# Run all tests
cargo test

# Run lib tests only
cargo test --lib

# Platform-specific testing required on actual Windows/macOS systems
```

### Alternative: PowerShell Script (Windows only, no compilation needed)
```powershell
cd scripts
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
.\wecom-multi-open-simple.ps1 -Count 2
```

## Architecture

### Cross-Platform Abstraction Layer

The codebase uses conditional compilation to support multiple platforms:

```
src/
├── lib.rs     # Core library with platform modules
│   ├── platform::windows (Windows-specific)
│   ├── platform::macos (macOS-specific)
│   └── platform::unsupported (other platforms)
└── main.rs    # CLI entry point using lib
```

**Key Design**: All platform-specific code is isolated in `src/lib.rs` under `#[cfg(target_os = "xxx")]` blocks, providing unified API:
- `spawn_multiple(req) -> SpawnResponse` - Launch multiple instances
- `kill_process(pid)` - Terminate a process
- `process_exists(pid)` - Check if process is running
- `get_default_app_path()` - Get platform-specific default path

### Platform-Specific Implementation

**Windows** (`#[cfg(target_os = "windows")]`):
- Uses `NtQuerySystemInformation` to enumerate system handles
- Calls `DuplicateHandle` with `DUPLICATE_CLOSE_SOURCE` to close mutex `Tencent.WeWork.ExclusiveObject`
- Uses `CreateProcessW` to launch instances
- Timing: 100ms wait after mutex close, 800ms between instances

**macOS** (`#[cfg(target_os = "macos")]`):
- Simply uses `open -n` command (macOS natively supports multiple instances)
- No mutex manipulation needed
- Timing: 500ms between instances

**Unsupported platforms**: Return error messages

### Data Structures

```rust
pub struct SpawnRequest {
    pub count: u8,
    pub app_path: Option<PathBuf>,
}

pub struct SpawnResponse {
    pub pids: Vec<u32>,
    pub success: usize,
    pub failed: usize,
}
```

## Critical Implementation Details

### Windows Mutex Handling
The Windows implementation operates at system level using undocumented NT APIs:
1. Allocates 64KB buffer for handle enumeration
2. Calls `NtQuerySystemInformation(SystemExtendedHandleInformation, ...)`
3. Iterates through all system handles
4. For each handle, uses `DuplicateHandle` with `DUPLICATE_CLOSE_SOURCE` flag to close it
5. This removes the mutex that prevents multiple instances

**Important**: Requires appropriate privileges (may need admin rights on Windows).

### Timing Parameters
These are empirically determined values critical to reliability:
- **100ms** after mutex close: Allows OS to release the mutex
- **800ms** between Windows instances: Allows process to fully initialize and recreate its own mutex
- **500ms** between macOS instances: Simpler, just prevents rapid-fire launches

## Development Workflows

### Adding Platform Support
When adding support for a new platform:
1. Add new `#[cfg(target_os = "platform")]` block in `src/lib.rs`
2. Implement required functions: `spawn_multiple`, `kill_process`, `process_exists`, `get_default_app_path`
3. Add platform-specific dependencies to `Cargo.toml` under `[target.'cfg(...)'.dependencies]`
4. Update documentation in relevant `.md` files
5. Add to GitHub Actions workflow (`.github/workflows/build.yml`)

### Modifying Core Logic
The spawn logic in both platforms follows: `for count { close_resources(); wait(); launch(); wait(); }`

When changing timing or logic:
- Windows users may need admin privileges
- Test with different instance counts (2, 3, 5, 10)
- Verify instances are truly independent (different accounts can login)
- Check system resource usage

### Cross-Compilation

The project supports building for different targets via GitHub Actions.

**Quick Windows Compilation Check on macOS:**
```bash
# One-command check for Windows compilation errors
./check-windows.sh
```

This script:
- Installs `x86_64-pc-windows-msvc` target if needed
- Checks all Rust code compiles for Windows
- Runs in ~10 seconds (vs 15 minutes for GitHub Actions)
- Does NOT build full installer (requires Windows environment)
- Perfect for catching compilation errors before pushing

**Manual cross-compilation:**
```bash
# Install Windows target
rustup target add x86_64-pc-windows-msvc

# Check library code
cargo check --target x86_64-pc-windows-msvc --lib

# Check CLI
cargo check --target x86_64-pc-windows-msvc --bin wecom-multi-open-cli

# Check GUI (will fail at linking but validates Rust syntax)
cargo check --target x86_64-pc-windows-msvc --features gui
```

**Limitations:**
- Cannot build full Windows installer on macOS (needs WiX toolset)
- GUI linking requires Windows resources (llvm-rc)
- Final testing must be done on actual Windows machine or GitHub Actions

## Key Files

- `src/lib.rs` - All platform-specific implementation (~300 lines)
- `src/main.rs` - CLI entry point (~40 lines)
- `Cargo.toml` - Defines library + binary structure with optional GUI feature
- `.github/workflows/build.yml` - Automated cross-platform builds
- `scripts/wecom-multi-open-simple.ps1` - Fallback PowerShell implementation

## Application Paths

**Windows**:
- `C:\Program Files (x86)\WXWork\WXWork.exe`
- `C:\Program Files\WXWork\WXWork.exe`

**macOS**:
- `/Applications/WeCom.app/Contents/MacOS/WeCom`

## Future GUI Implementation

The `Cargo.toml` includes a `gui` feature with optional Tauri dependency:
```toml
[features]
gui = ["tauri"]

[[bin]]
name = "wecom-multi-open-gui"
required-features = ["gui"]
```

Currently only CLI is implemented. GUI would follow layered architecture from `docs/dev.md`:
- Presentation Layer: Tauri + React
- Service Layer: Rust (current lib.rs)
- No modification needed to core platform code

## Release Process

Per `docs/dev.md` milestones:
1. ✅ MVP - CLI + PowerShell script (completed v0.1.0)
2. ✅ Cross-platform - Windows + macOS (completed v0.2.0)
3. ⏳ GUI - Tauri interface (planned)
4. ⏳ System tray support (planned)

To trigger automated builds:
```bash
git tag v0.2.0
git push origin v0.2.0
# GitHub Actions builds Windows .exe and macOS binaries
```

## Documentation Rules

**IMPORTANT**: Keep documentation minimal and focused on development needs only.

### What to Document
- ✅ Technical implementation details (in CLAUDE.md or inline comments)
- ✅ Critical platform-specific behavior
- ✅ API usage and architecture decisions
- ✅ Build and deployment workflows (brief notes only)

### What NOT to Document
- ❌ User guides / tutorials / manuals
- ❌ Testing guides / QA procedures
- ❌ Installation instructions
- ❌ FAQ / troubleshooting guides
- ❌ Multiple README files
- ❌ Detailed process documentation
- ❌ Marketing / feature descriptions

### Documentation Structure
```
docs/
├── dev.md                  # Core development guide (keep minimal)
├── DEVELOPMENT.md          # Architecture and implementation notes
└── [platform]-research.md  # Research findings (when needed)
```

**Rule**: Never create more than 3-4 documentation files. Keep each file under 300 lines. Update existing docs instead of creating new ones.

## UI/Frontend Development

### Tech Stack
- **Framework**: React 18 + TypeScript + Vite
- **UI Components**: Shadcn/ui (copy-paste components)
- **Styling**: Tailwind CSS
- **Desktop**: Tauri 2.x
- **Icons**: Lucide React
- **Animation**: Framer Motion

### Project Structure
```
ui/
├── src/
│   ├── components/ui/     # Shadcn components (keep original)
│   ├── styles/
│   │   └── globals.css    # Global styles and overrides
│   ├── hooks/             # Custom React hooks
│   ├── lib/               # Utilities
│   └── App.tsx            # Main application
├── package.json
└── vite.config.ts
```

### Styling Guidelines

**IMPORTANT**: Use global styles for design system modifications, NOT direct component edits.

#### Design Philosophy
- **Flat Design**: Large border radius (16px), minimal shadows
- **Clean Interface**: Remove borders/shadows from non-outline components
- **Consistency**: Centralized design tokens in CSS variables

#### CSS Architecture

```css
/* ui/src/styles/globals.css */

:root {
  /* Design Tokens - Modify these for global changes */
  --radius: 16px;           /* Border radius */
  --font-sans: ...;         /* Typography */
  /* Color tokens... */
}

@layer components {
  /* Component-level style overrides */
  /* Use this for removing borders, shadows, etc. */
}

@layer utilities {
  /* Custom utility classes */
}
```

#### Styling Best Practices

**✅ DO:**
1. **Modify design tokens in `:root`** for global changes:
   ```css
   :root {
     --radius: 16px;  /* Affects all components */
   }
   ```

2. **Override component styles in `@layer components`**:
   ```css
   @layer components {
     /* Remove Card borders and shadows */
     [class*="rounded-"][class*="border"][class*="bg-card"] {
       @apply border-0 shadow-none;
     }

     /* Remove Button shadows (except outline) */
     button[class*="bg-primary"][class*="shadow"] {
       @apply shadow-none;
     }
   }
   ```

3. **Keep shadcn components in original state**:
   - Components in `ui/src/components/ui/` should NOT be modified
   - This allows easy upgrades and maintains consistency
   - All customizations go in `globals.css`

**❌ DON'T:**
1. ❌ Directly edit component files (`button.tsx`, `card.tsx`, etc.)
2. ❌ Add inline styles in component props
3. ❌ Create duplicate components for minor style changes
4. ❌ Use `!important` to override styles

#### Why Global Styles Over Component Edits?

**Advantages of Global Approach:**
- ✅ **Centralized Design System**: All visual styles in one place
- ✅ **Easy Upgrades**: Shadcn components stay pristine, can be updated easily
- ✅ **Maintainability**: Single source of truth for design decisions
- ✅ **Git Cleanliness**: Fewer component file modifications in version control
- ✅ **Tailwind Layers**: Proper use of `@layer` system for predictable specificity

**Problems with Direct Component Edits:**
- ❌ Styles scattered across multiple files
- ❌ Difficult to upgrade shadcn components
- ❌ Hard to track what's been customized
- ❌ Need higher specificity to override later

#### Example: Flat Design Implementation

```css
/* globals.css - Centralized flat design */

:root {
  --radius: 16px;  /* Larger, flatter corners */
}

@layer components {
  /* Card: Remove border and shadow for flat look */
  [class*="rounded-"][class*="border"][class*="bg-card"][class*="shadow"] {
    @apply border-0 shadow-none;
  }

  /* Buttons: Remove shadows (keep outline border) */
  button[class*="bg-primary"][class*="shadow"],
  button[class*="bg-destructive"][class*="shadow"],
  button[class*="bg-secondary"][class*="shadow"] {
    @apply shadow-none;
  }

  /* Outline buttons: Keep border, remove shadow */
  button[class*="border"][class*="border-input"] {
    @apply border shadow-none;
  }
}
```

#### Component Usage Patterns

**Tabs for Navigation:**
```tsx
import { Tabs, TabsList, TabsTrigger } from './components/ui/tabs'

<Tabs value={current} onValueChange={setCurrent}>
  <TabsList className="grid w-full grid-cols-2">
    <TabsTrigger value="option1">Option 1</TabsTrigger>
    <TabsTrigger value="option2">Option 2</TabsTrigger>
  </TabsList>
</Tabs>
```

**Platform-Specific Styling:**
```tsx
// Detect platform in component
const [platform, setPlatform] = useState<'windows' | 'macos' | 'other'>('other')

useEffect(() => {
  const platformStr = navigator.platform.toLowerCase()
  if (platformStr.includes('mac')) {
    document.documentElement.classList.add('platform-macos')
    setPlatform('macos')
  } else if (platformStr.includes('win')) {
    document.documentElement.classList.add('platform-windows')
    setPlatform('windows')
  }
}, [])

// Conditional rendering
{platform === 'windows' && <WindowsOnlyFeature />}
{platform === 'macos' && <MacOnlyFeature />}
```

**Glass Effect (macOS/Windows):**
```tsx
<Card className="glass-effect">
  {/* Automatic platform-specific backdrop-filter */}
</Card>
```

### Build Process

**Development:**
```bash
cd ui
npm run dev       # Vite dev server
npm run build     # Production build
```

**Full App Build:**
```bash
# From project root
cargo tauri dev   # Development mode with hot reload
cargo tauri build # Production build (platform-specific installer)
```

### Dependencies

**When adding UI components:**
```bash
cd ui
npm install @radix-ui/react-[component]  # Radix primitives for shadcn
```

**Current shadcn components:**
- Button
- Card
- Tabs
- (Add more as needed via copy-paste from shadcn/ui docs)
