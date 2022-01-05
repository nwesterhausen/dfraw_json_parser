let database = [];
fetch("./out.json")
  .then((response) => {
    setStatus("Ready");
    return response.json();
  })
  .then((data) => {
    document.getElementById(
      "results"
    ).innerHTML = `<p class="container">${data.length} creatures</p>`;
    database = data;
    database = data.map(copyFromIfNeeded);
  });
const searchBar = document.getElementById("searchbar");
searchBar.addEventListener("keyup", (e) => {
  const searchString = e.target.value;

  setStatus("Searching");
  new Promise((resolve, reject) => {
    const filteredCreatures = database.filter((creature) => {
      return (
        creature.name.includes(searchString) ||
        creature.description.includes(searchString) ||
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
      document.getElementById("results").innerHTML = displayCards;
      setStatus("Ready");
    });
});
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
function creatureToHTML(creature) {
  return `<div class="card" style="width: 20rem;">
<div class="card-body">
<h5 class="card-title">${creature.name.split(":")[0]}</h5>
<p class="text-muted">${creature.name.split(":").splice(0, 1).join(", ")}</p>
<p class="card-text">${creature.description}</p>
</div>
<ul class="list-group list-group-flush">
<li class="list-group-item">Lives ${creature.max_age.join(" - ")} years</li>
<li class="list-group-item">${
    creature.lays_eggs
      ? "Lays " + creature.clutch_size.join(" - ") + " eggs per clutch"
      : "Doesn't lay eggs."
  }</li>
</ul>
<div class="card-body text-muted text-small">
<h6>Rawfile: <strong>${creature.parent_raw}</strong></h6>
<h6>ID: <strong>${creature.identifier}</strong></h6>
</div>
</div>`;
}
function displayCreatures(creatureArr) {
  return `<div class="container"><div class="row row-cols-3">
      <div class="col">${creatureArr
        .map(creatureToHTML)
        .join('</div><div class="col">')}</div>
      </div></div>`;
}
function copyFromIfNeeded(creature) {
  if (!creature.based_on) {
    return creature;
  }
  let baseCreature = database.find((s) => s.objectId === creature.based_on);
  if (!baseCreature) {
    return creature;
  }
  console.info(`Copying info from ${baseCreature.name} to ${creature.name}`);
  if (
    creature.max_age[0] === creature.max_age[1] &&
    creature.max_age[0] === 0
  ) {
    creature.max_age = baseCreature.max_age;
  }

  return creature;
}
