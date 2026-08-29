import init, { synthesize_with_annotation } from '../pkg/winnow_core'

async function main() {
  await init()
  const positives = ["Buy milk", "Buy eggs"]
  const negatives = ["Call mom"]
  const result = synthesize_with_annotation(positives, negatives)
  console.log("Generated pattern:", result)
  document.body.innerHTML = `<pre>Generated: ${result}</pre>`
}

main()
