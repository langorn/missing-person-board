import { useEffect, useState } from "react";
import { Bulletin } from "./Bulletin";


const PER_PAGE_LIMIT = 1;

const BullentinList = ({ contract }) => {
    const [bulletins, setBulletins] = useState([]);
    const [page, setPage] = useState(1);

    useEffect(() => {
        let offset;
        if (page <1) {
            setPage(1);
            offset = 0;
        } else {
            offset = (page - 1) * PER_PAGE_LIMIT;
        }
        offset = String(offset);
            contract
            .get_bulletins({from_index: offset, limit: PER_PAGE_LIMIT})
            .then((bulletins) => { 
                console.log(bulletins);
                setBulletins(bulletins)});
    }, [page, contract]);

    return (
        <ul>
          <div className="flex">
          Current Page: {page}
          </div>
          <button onClick={() => setPage((page) => page - 1)}>&lt;</button>
          {" "}
          <button onClick={() => setPage((page) => page + 1)}>&gt;</button>
          {bulletins.map((bulletin) => (
            <li key={bulletin.id}>
              <Bulletin contract={contract} {...bulletin} />
            </li>
          ))}
        </ul>
      );

}

export default BullentinList;