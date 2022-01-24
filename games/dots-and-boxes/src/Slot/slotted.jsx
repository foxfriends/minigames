import React from "react";
import Slot from "./Slot.jsx";

function getSlots(children) {
  if (!Array.isArray(children)) {
    return {};
  }
  return children
    .filter((child) => child.type === Slot)
    .reduce((slots, child) => {
      slots[child.props.name] = child;
      return slots;
    }, {});
}

export default function slotted(Component) {
  return function Slotted({ children, ...props }) {
    const slots = getSlots(children);
    const restChildren = Array.isArray(children)
      ? children.filter((child) => child.type !== Slot)
      : children;

    return (
      <Component {...props} slots={slots}>
        {restChildren}
      </Component>
    );
  };
}
