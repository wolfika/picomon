<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    title,
    aside,
    class: klass = '',
    children,
  }: { title: string; aside?: string; class?: string; children: Snippet } = $props();
</script>

<section class="card {klass}">
  <header>
    <h2>{title}</h2>
    {#if aside}
      <span class="aside" title={aside}>{aside}</span>
    {/if}
  </header>
  {@render children()}
</section>

<style>
  .card {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 18px;
    min-width: 0;
    padding: 22px 24px 24px;
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
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: var(--muted);
  }
  .aside {
    font-size: 12px;
    color: var(--faint);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
