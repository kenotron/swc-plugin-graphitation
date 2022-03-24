export const query = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "Foo", loc: undefined },
      variableDefinitions: [],
      directives: [],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: undefined,
            name: { kind: "Name", value: "foo", loc: undefined },
            arguments: [],
            directives: [],
            selectionSet: undefined,
            loc: undefined,
          },
        ],
        loc: undefined,
      },
      loc: undefined,
    },
  ].concat([]),
};
