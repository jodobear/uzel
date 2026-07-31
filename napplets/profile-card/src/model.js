export {
  canonicalProfile,
  MAXIMUM_DATE_SECONDS,
  profileQueryRequest,
  PROFILE_RESULT_LIMIT,
} from '../../../contracts/kind0-profile.js';

export function createLatestRequestGate() {
  let generation = 0;
  return Object.freeze({
    begin() {
      generation += 1;
      return generation;
    },
    isCurrent(requestGeneration) {
      return requestGeneration === generation;
    },
  });
}
