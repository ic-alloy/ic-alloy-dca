export function ethStringToDecimal(eth: string) {
  const ethBigInt = BigInt(eth);
  const divisor = BigInt(10 ** 18);
  const wholePart = ethBigInt / divisor;
  const fractionalPart = ethBigInt % divisor;
  const fractionalPartStr = fractionalPart.toString().padStart(18, '0').slice(0, 4);
  return `${wholePart.toString()}.${fractionalPartStr}`;
}

export function decimalStringToEth(amount: string) {
  amount = amount.replace(',', '.');
  const amountAsNumber = parseFloat(amount);
  return BigInt(Math.round(amountAsNumber * 10 ** 18));
}
