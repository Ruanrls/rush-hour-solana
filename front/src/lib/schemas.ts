export const boardToSolanaSchema = {
  struct: {
    board: {
      array: {
        type: {
          array: {
            type: "u8",
          },
        },
      },
    },
  },
};

export const solanaResultSchema = {
  struct: {
    result: {
      array: {
        type: {
          struct: {
            id: "u8",
            direction: "u8",
          },
        },
      },
    },
  },
};
