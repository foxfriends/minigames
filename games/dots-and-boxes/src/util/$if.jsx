export default function $if(bool, component) {
  return bool ? component() : null;
}
