<script lang="ts">
  import { tick } from 'svelte';
  import { THEMES, serverTheme, setTheme } from './theme';

  let current = $state(document.documentElement.dataset.theme ?? serverTheme);
  let open = $state(false);
  let root: HTMLElement;
  let button: HTMLButtonElement;
  let menu = $state<HTMLElement>();

  function items(): HTMLElement[] {
    return menu ? Array.from(menu.querySelectorAll<HTMLElement>('[role="menuitemradio"]')) : [];
  }

  async function show(focus: 'current' | 'first' | 'last' = 'current') {
    open = true;
    await tick();
    const all = items();
    const target =
      focus === 'first' ? all[0] : focus === 'last' ? all.at(-1) : all[THEMES.indexOf(current as never)];
    (target ?? all[0])?.focus();
  }

  function hide(returnFocus = true) {
    open = false;
    if (returnFocus) button.focus();
  }

  function choose(t: string) {
    current = t;
    setTheme(t);
    hide();
  }

  function onButtonKey(e: KeyboardEvent) {
    if (e.key === 'ArrowDown') show('first');
    else if (e.key === 'ArrowUp') show('last');
    else return;
    e.preventDefault();
  }

  function onMenuKey(e: KeyboardEvent) {
    const all = items();
    const i = all.indexOf(document.activeElement as HTMLElement);
    if (e.key === 'ArrowDown') all[(i + 1) % all.length].focus();
    else if (e.key === 'ArrowUp') all[(i - 1 + all.length) % all.length].focus();
    else if (e.key === 'Home') all[0].focus();
    else if (e.key === 'End') all[all.length - 1].focus();
    else if (e.key === 'Escape') hide();
    else if (e.key === 'Tab') hide(false);
    else return;
    if (e.key !== 'Tab') e.preventDefault();
  }

  function onWindowPointer(e: PointerEvent) {
    if (open && !root.contains(e.target as Node)) hide(false);
  }
</script>

<svelte:window onpointerdown={onWindowPointer} />

<div class="theme-menu" bind:this={root}>
  <button
    bind:this={button}
    class="icon-button"
    type="button"
    aria-label="Theme"
    title="Theme"
    aria-haspopup="menu"
    aria-expanded={open}
    aria-controls="theme-menu-list"
    onclick={() => (open ? hide() : show())}
    onkeydown={onButtonKey}
  >
    <svg viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
      <circle cx="12" cy="12" r="8.5" fill="none" stroke="currentColor" stroke-width="2" />
      <path d="M12 3.5a8.5 8.5 0 0 1 0 17z" fill="currentColor" />
    </svg>
  </button>

  {#if open}
    <div id="theme-menu-list" class="menu" role="menu" aria-label="Theme" tabindex="-1" bind:this={menu} onkeydown={onMenuKey}>
      {#each THEMES as t (t)}
        <button
          type="button"
          role="menuitemradio"
          aria-checked={t === current}
          tabindex="-1"
          onclick={() => choose(t)}
        >
          <!-- The swatch carries its own data-theme, so it previews that preset's colours. -->
          <span class="swatch" data-theme={t} aria-hidden="true"><i></i></span>
          <span class="name">{t}</span>
          {#if t === serverTheme}<span class="hint">default</span>{/if}
          <svg class="check" viewBox="0 0 24 24" width="14" height="14" aria-hidden="true">
            <path d="M5 12.5l4.5 4.5L19 7.5" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .theme-menu {
    position: relative;
  }
  .icon-button {
    display: inline-grid;
    place-items: center;
    width: 30px;
    height: 30px;
    padding: 0;
    border-radius: 999px;
    border: 1px solid var(--border);
    background: var(--surface);
    backdrop-filter: var(--blur);
    -webkit-backdrop-filter: var(--blur);
    color: var(--text);
    cursor: pointer;
  }
  .icon-button:hover,
  .icon-button[aria-expanded='true'] {
    border-color: color-mix(in srgb, var(--text) 30%, transparent);
  }
  .icon-button:focus-visible,
  .menu button:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
  .menu {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    z-index: 10;
    min-width: 180px;
    padding: 6px;
    border-radius: 12px;
    border: 1px solid var(--border);
    background: var(--tooltip);
    box-shadow: var(--shadow), 0 12px 32px rgb(0 0 0 / 0.25);
    backdrop-filter: var(--blur);
    -webkit-backdrop-filter: var(--blur);
  }
  .menu button {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 7px 8px;
    border: 0;
    border-radius: 8px;
    background: none;
    color: var(--text);
    font: inherit;
    font-size: 13px;
    text-align: left;
    cursor: pointer;
  }
  .menu button:hover,
  .menu button:focus-visible {
    background: color-mix(in srgb, var(--text) 10%, transparent);
  }
  .swatch {
    display: grid;
    place-items: center;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background-color: var(--bg);
    background-image: var(--bg-image);
    border: 1px solid color-mix(in srgb, var(--muted) 50%, transparent);
  }
  .swatch i {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent);
  }
  .name {
    text-transform: capitalize;
  }
  .hint {
    font-size: 11px;
    color: var(--muted);
  }
  .check {
    margin-left: auto;
    visibility: hidden;
  }
  [aria-checked='true'] .check {
    visibility: visible;
  }
</style>
