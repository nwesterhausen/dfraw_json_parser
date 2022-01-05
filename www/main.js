// define a database to store all the records in
let database = [];

// grab the generated .json file to build the list
fetch("./out.json")
  .then((response) => {
    // convert from string to json
    return response.json();
  })
  .then((data) => {
    database = data;
    // populate records that have specified a COPY_FROM tag
    database = data.map(copyFromIfNeeded);
    setStatus("Ready");
  })
  .catch((err) => {
    window.alert(`Unable to grab the out.json or parse it correctly.\n${err.message}`);
  });

// define the search bar and attach a listener
const searchBar = document.getElementById("searchbar");
searchBar.addEventListener("keyup",delay((e) => {
  // when you type in teh searchbar, it will begin filtering results
  const searchString = e.target.value;

  setStatus("Searching");
  new Promise((resolve, reject) => {
    // perform search filters
    const filteredCreatures = database.filter((creature) => {
      return (
        // check if the search string is in the name
        creature.name.includes(searchString) ||
        // check if the search string is in the name
        creature.description.includes(searchString) ||
        // check if the search string is egg(s) to display all egg_layers
        ((searchString.toLowerCase() === "egg" ||
          searchString.toLowerCase() === "eggs") &&
          creature.lays_eggs)
      );
    });
    return resolve(filteredCreatures);
  })
    .then((filteredCreatures) => {
      return displayCreatures(filteredCreatures);
    })
    .then((displayCards) => {
      // after building the HTML to populate the results, insert it into the page
      document.getElementById("results").innerHTML = displayCards;
      setStatus("Ready");
    });
}), 250);

/**
 * Change the status banner on the navbar. Displays as "warning" unless message is "Ready"
 * @param {string} message what status to display
 */
function setStatus(message) {
  let statusField = document.getElementById("statusfield");
  if (message !== "Ready") {
    statusField.classList.remove("text-success");
    statusField.classList.add("text-warning");
  } else {
    statusField.classList.remove("text-warning");
    statusField.classList.add("text-success");
  }
  console.info(`Current status ${message}`);
  statusField.innerText = message;
}

/**
 * The delay function will return a wrapped function that internally handles an 
 * individual timer, in each execution the timer is restarted with the time delay 
 * provided, if multiple executions occur before this time passes, the timer will 
 * just reset and start again.
 * 
 * When the timer finally ends, the callback function is executed, passing the original 
 * context and arguments (in this example, the jQuery's event object, and the DOM element 
 * as this).
 * 
 * @param {function} callback function to delay calling
 * @param {number} ms time to delay in ms
 * @returns {function} wrapped function
 */
 function delay(fn, ms) {
  let timer = 0
  return function(...args) {
    clearTimeout(timer)
    timer = setTimeout(fn.bind(this, ...args), ms || 0)
  }
}