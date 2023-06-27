import re

text = "The quick brown fox jumps over the lazy dog. Apex and Index are important terms in programming."
pattern = r'\w*ex\b'
matches = re.findall(pattern, text)

print(matches)  # Output: ['Apex', 'Index']

