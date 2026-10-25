export {};

declare module "@mrlyprod/mrly/math" {
  namespace shape {
    function crop(types: Tensor, shape: Shape | { Anti: Shape }, keep_cut: boolean): Tensor;
    function census(shape: Shape | { Anti: Shape }, types: Tensor): ShapeCensus;
    function regions(shape: Shape | { Anti: Shape }, dims: ArrayLike<number>): Tensor;
    function refine(
      types: Tensor,
      shape: Shape | { Anti: Shape },
      base: number,
      extra: number,
      keep_cut: boolean,
    ): Tensor;
  }
}
