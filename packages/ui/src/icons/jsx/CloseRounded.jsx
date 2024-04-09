import * as React from "react";
const SvgCloseRounded = (props) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="16px"
    height="16px"
    fill="none"
    viewBox="0 0 16 16"
    {...props}
  >
    <path
      stroke="#000"
      strokeMiterlimit={10}
      strokeWidth={0.9}
      d="M8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12Z"
    />
    <path
      stroke="#000"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth={0.9}
      d="m10 6-4 4M10 10 6 6"
    />
  </svg>
);
export default SvgCloseRounded;
