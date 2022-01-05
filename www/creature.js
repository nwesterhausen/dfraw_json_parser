/**
 * Turn a Creature object into a card for display purposes.
 * @param {Creature} creature 
 * @returns html string of creature card
 */
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

/**
 * Turn an array of creature cards into an array
 * @param {Creature[]} creatureArr an array of Creature objects
 * @returns html string of all creatures as cards for display
 */
function displayCreatures(creatureArr) {
  return `<div class="container"><div class="row row-cols-3">
        <div class="col">${creatureArr
          .map(creatureToHTML)
          .join('</div><div class="col">')}</div>
        </div></div>`;
}

/**
 * Update Creature object with the information stored in the COPY_FROM creature
 * @param {Creature} creature 
 * @returns creature after copying missing attributes from the database
 */
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
