/**
 * Turn a Creature object into a card for display purposes.
 * @param {Creature} creature
 * @returns html string of creature card
 */
function creatureToHTML(creature) {
  return `<div class="card m-1" style="width: 20rem;">
  <div class="card-body">
  <h5 class="card-title">${creature.names[0]}</h5>
  <p class="text-muted">${creature.names.join(", ")}</p>
  <p class="card-text">${creature.description}</p>
  </div>
  <ul class="list-group list-group-flush">
  <li class="list-group-item">${maxAgeStatus(creature)}</li>
  <li class="list-group-item">${eggLayingStatus(creature)}</li>
  </ul>
  <div class="card-body text-muted text-small">
  <h6>Rawfile: <strong>${creature.parent_raw}</strong></h6>
  <h6>ID: <strong>${creature.identifier}</strong></h6>
  </div>
  </div>`;
}

/**
 * Get the max age string for a creature.
 * @param {Creature} creature 
 * @returns string for displaying max age information
 */
function maxAgeStatus(creature) {
  if (Object.keys(creature.max_age).length === 0) {
    return "Lives indefinitely.";
  }
  let ret = "";
  for (let k in creature.max_age) {
    ret += `${k} lives ${creature.max_age[k].join(" - ")} years.`
  }
  return ret;
}

/**
 * Get the egg laying string for a creature.
 * @param {Creature} creature 
 * @returns string for displaying egg-laying information
 */
function eggLayingStatus(creature) {
  if (!creature.lays_eggs) {
    return "Doesn't lay eggs.";
  }
  let ret = "";
  for (let k in creature.clutch_size) {
    ret += `${k} lays ${creature.clutch_size[k].join(" - ")} eggs.`
  }
  return ret;
}

/**
 * Turn an array of creature cards into an array
 * @param {Creature[]} creatureArr an array of Creature objects
 * @returns html string of all creatures as cards for display
 */
function displayCreatures(creatureArr) {
  return `<div class="container-fluid"><div class="d-flex flex-wrap">
        ${creatureArr
          .map(creatureToHTML)
          .join('')}
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
  console.info(`Copying info from ${baseCreature.objectId} to ${creature.objectId}`);
  if (
    Object.keys(creature.max_age).length === 0
  ) {
    creature.max_age = baseCreature.max_age;
  }

  return creature;
}