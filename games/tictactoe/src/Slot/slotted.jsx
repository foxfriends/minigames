import React from "react";
import Slot from "./Slot.jsx";

export default function slotted(Component) {
  return function Slotted({ children, ...props }) {
    const slots = children
      .filter((child) => child.type === Slot)
      .reduce((slots, child) => {
        slots[child.props.name] = child;
        return slots;
      }, {});
    const restChildren = children.filter((child) => child.type !== Slot);

    return (
      <Component {...props} slots={slots}>
        {restChildren}
      </Component>
    );
  };
}
