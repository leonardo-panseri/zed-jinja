(definition
  (raw_block
    (raw_body) @injection.content
    (#set! injection.language "Html")))

((words)+ @injection.content
    (#set! injection.language "Html")
    (#set! injection.combined "true"))
