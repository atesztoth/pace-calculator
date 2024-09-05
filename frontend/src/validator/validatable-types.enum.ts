export enum ValidatorKey {
  /**
   * @description For everything that has a shape of (xx(:yy(:zz))) where x,y,z are nat. numbers
   */
  stringTime,

  /**
   * @description stringTime that has a value gt 0
   */
  stringPace,

  /**
   * @description Input is a stringified number.
   */
  stringNumber,

  /**
   * @description Positive number. That's all.
   */
  stringNumberGtZero,
}
