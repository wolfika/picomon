<script lang="ts">
  import type { Snippet } from 'svelte';

  let { title, aside, children }: { title: string; aside?: string; children: Snippet } = $props();
</script>

<section class="card">
  <header>
    <h2>{title}</h2>
    {#if aside}<span class="aside" title={aside}>{aside}</span>{/if}
  </header>
  {@render children()}
</section>

<style>
  .card {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 14px;
    min-width: 0;
    padding: 18px 20px 20px;
    border-radius: var(--radius);
    background: var(--surface);
    border: 1px solid var(--border);
    box-shadow: var(--shadow);
    backdrop-filter: var(--blur);
    -webkit-backdrop-filter: var(--blur);
  }
  header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
    min-width: 0;
  }
  h2 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--muted);
  }
  .aside {
    font-size: 12px;
    color: var(--muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
