<script lang="ts">
	export let name = '';

	export let small = false;
	export let disabled = false;
	export let group = '';
	export let value = '';
</script>

<input
	on:click|stopPropagation
	on:change
	on:input
	type="radio"
	class="radio"
	class:small
	{value}
	{name}
	{disabled}
	bind:group
/>

<style lang="postcss">
	.radio {
		appearance: none;
		cursor: pointer;
		width: var(--space-16);
		height: var(--space-16);
		border-radius: var(--space-16);
		background-color: var(--clr-theme-container-light);
		box-shadow: inset 0 0 0 1px var(--clr-theme-container-outline-light);
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast),
			opacity var(--transition-fast),
			transform var(--transition-fast);
		position: relative;

		/* not checked */
		&:hover,
		&:focus {
			box-shadow: inset 0 0 0 1px
				color-mix(in srgb, var(--clr-theme-container-outline-pale), var(--darken-mid));
			outline: none;

			&::after {
				opacity: 0.2;
				transform: scale(0.7);
			}
		}

		&:disabled {
			pointer-events: none;
			opacity: 0.3;
			cursor: not-allowed;
			background-color: var(--clr-theme-scale-ntrl-60);
			border-color: none;
		}

		/* checked */
		&:checked {
			background-color: var(--clr-theme-pop-element);
			box-shadow: inset 0 0 0 1px var(--clr-theme-pop-element);

			&:hover {
				background-color: color-mix(in srgb, var(--clr-theme-pop-element), var(--darken-mid));
			}

			&:disabled {
				pointer-events: none;
				opacity: 0.4;
				cursor: not-allowed;
			}

			&::after {
				opacity: 1;
				transform: scale(1);
				background-color: var(--clr-core-ntrl-100);
			}
		}

		/* tick element */
		&::after {
			content: '';
			position: absolute;
			top: var(--space-4);
			left: var(--space-4);
			width: calc(100% - var(--space-8));
			height: calc(100% - var(--space-8));
			border-radius: var(--space-16);
			background-color: var(--clr-theme-scale-ntrl-0);
			transition: background-color var(--transition-fast);
			transform: scale(0.5);
			opacity: 0;
		}

		/* modifiers */

		&.small {
			width: var(--space-14);
			height: var(--space-14);
		}
	}
</style>
