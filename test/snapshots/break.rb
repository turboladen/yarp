CallNode(
  CallNode(
    nil,
    nil,
    IDENTIFIER("foo"),
    nil,
    nil,
    nil,
    BlockNode(
      BlockParametersNode(
        ParametersNode(
          [RequiredParameterNode(IDENTIFIER("a"))],
          [],
          nil,
          [],
          nil,
          nil
        ),
        []
      ),
      StatementsNode([BreakNode(nil, (155..160))]),
      (149..150),
      (161..162)
    ),
    "foo"
  ),
  nil,
  EQUAL_EQUAL("=="),
  nil,
  ArgumentsNode([IntegerNode()]),
  nil,
  nil,
  "=="
)