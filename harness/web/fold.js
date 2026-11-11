// The overview's definition, folded: its first sentence shown and the line after it fading out,
// and an arrow under it that shows the rest and folds it again.  Folded is how the stylesheet
// draws it, at the height the full measure gives; here that height is set for the measure in
// hand, where the first sentence runs to more lines or fewer.
const text = document.getElementById('definition');
const first = text.querySelector('.first');
const button = document.querySelector('button.unfold');

/** The folded height: the lines the first sentence runs to and one more. */
function fold() {
  if (text.classList.contains('whole')) { text.style.maxHeight = ''; return; }
  const line = parseFloat(getComputedStyle(first).lineHeight);
  const rects = first.getClientRects();
  const last = rects[rects.length - 1];
  const lines = Math.round((last.top - text.getBoundingClientRect().top) / line) + 1;
  text.style.maxHeight = `${(lines + 1) * line}px`;
}

button.addEventListener('click', () => {
  const whole = text.classList.toggle('whole');
  button.textContent = whole ? '↑' : '↓';
  button.setAttribute('aria-expanded', whole);
  button.setAttribute('aria-label', whole ? 'Fold the definition' : 'Show the whole definition');
  fold();
});
new ResizeObserver(fold).observe(text);
fold();
