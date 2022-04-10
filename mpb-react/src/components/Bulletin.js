import { useState } from "react";

export function Bulletin({ id, sender, img_url, location, description, contact, done }) {
  return (
    <>
      <div className="border">
        <div>{id}</div>
        <div>{sender}</div>
        <div>{img_url}</div>
        <div>{location}</div>
        <div>{description}</div>
        <div>{contact}</div>
      </div>
    </> 

  );
}
export default Bulletin;