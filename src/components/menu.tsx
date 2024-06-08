import { Fragment } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHome, faUserCog, faBook } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";

function Menu() {
  return (
    <Fragment>
      <ul>
        {[
          { id: 1, href: "/", icon: faHome, title: "Home" },
          { id: 2, href: "/settings", icon: faUserCog, title: "Settings" },
          { id: 3, href: "/", icon: faBook, title: "Docs" },
        ].map(({ id, href, icon, title }) => (
          <li
            key={id}
            className="rounded w-9 h-9 mt-3 mb-7 ms-auto me-auto bg-gray-300 dark:bg-slate-700/75 dark:text-white hover:bg-emerald-700 hover:text-white"
          >
            <Link
              to={href}
              className="flex flex-col items-center justify-center w-9 h-9"
              title={title}
            >
              <FontAwesomeIcon
                icon={icon}
                className="ml-auto mr-auto w-4 h-4"
              />
            </Link>
          </li>
        ))}
      </ul>
    </Fragment>
  );
}

export default Menu;
