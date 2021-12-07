import * as React from 'react';
import { cell } from './Cell.module.css';
import cond, { otherwise } from '../util/cond';
import equals from '../util/equals';

export default function Cell({ value }) {
  const content = cond([
    [equals('x'), () => '×'],
    [equals('o'), () => '○'],
    [otherwise, () => ''],
  ])(value);

  return (
    <div className={cell}>
      {content}
    </div>
  );
}
