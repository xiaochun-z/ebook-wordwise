import { Fragment, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHome, faMoon, faLightbulb } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";
import About from "./about";

function Menu() {
  function changeTheme() {
    if (darkTheme) {
      document.documentElement.classList.remove("dark");
    } else {
      document.documentElement.classList.add("dark");
    }

    setDarkTheme(!darkTheme);
  }

  const [darkTheme, setDarkTheme] = useState(true);

  return (
    <Fragment>
      <ul>
        {[
          {
            id: 1,
            href: "/",
            icon: () => faHome,
            clickHandler: () => { },
          },
          {
            id: 2,
            href: "#",
            icon: () => (darkTheme ? faLightbulb : faMoon),
            clickHandler: () => {
              changeTheme();
            },
          }
        ].map(({ id, href, icon, clickHandler }) => (
          <li
            key={id}
            className="rounded w-9 h-9 mt-3 mb-7 ms-auto me-auto bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-blue-800 dark:hover:text-blue-50 hover:text-blue-700"
          >
            <Link
              to={href}
              className="flex flex-col items-center justify-center w-9 h-9"
              data-modal-target="default-modal" data-modal-toggle="default-modal"
              onClick={clickHandler}
            >
              <FontAwesomeIcon
                icon={icon()}
                className="ml-auto mr-auto w-4 h-4"
              />
            </Link>
          </li>
        ))}
        <About />
      </ul>
    </Fragment>
  );
}

export default Menu;
