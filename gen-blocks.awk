# Script to regenerate src/unicode_blocks.rs from Blocks.txt
#
# Usage awk -f gen-blocks.awk Blocks.txt > src/unicode_blocks.rs
#
# where Blocks.txt is fetched from:
# https://www.unicode.org/Public/UNIDATA/Blocks.txt

BEGIN { FS=";" }

/^# Blocks-[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+\.txt/ {
  pos = match($0, "[[:digit:]]+\\.[[:digit:]]+\\.[[:digit:]]+");
  if (pos < 1) {
    printf("unable to extract version from: %s\n", $0);
    exit(1);
  }

  version = substr($0, RSTART, RLENGTH);

  print "// The dataset is from https://www.unicode.org/Public/UNIDATA/Blocks.txt"
  print ""
  print "use crate::UnicodeBlock;"
  print ""
  printf("pub const VERSION: &str = \"%s\";\n", version);

  idx = 1
}

# Only process non-comment lines
/^[[:digit:]A-F]/ {
  split($1, range, "\\.\\.");
  start = range[1];
  end = range[2];

  name = $2
  # trim leading spaces
  if (match(name, "^ +") > 0) {
    name = substr(name, RLENGTH + 1);
  }

  # build Rust constant name
  const = name
  gsub("[ -]", "_", const);
  const = toupper(const);

  printf("pub const %s: UnicodeBlock = UnicodeBlock {\n", const);
  printf("    name: \"%s\", start: 0x%s, end: 0x%s\n", name, start, end);
  print "};"

  # skip blocks that end in _SURROGATES as these are not valid as Unicode
  # escapes in Rust.
  if (match(const, "_SURROGATES$") < 1) {
    match_line = sprintf("        '\\u{%s}'..='\\u{%s}' => Some(%s),", start, end, const);
    match_lines[idx++] = match_line;
  }
}

END {
  print ""
  print "/// Given a character, determine what unicode block contains it."
  print "pub fn find_unicode_block(c: char) -> Option<UnicodeBlock> {"
  print "    match c {"
  for (i = 1; i < idx; i++) {
    print match_lines[i];
  }
  print "        _ => None,"
  print "    }"
  print "}"
}
