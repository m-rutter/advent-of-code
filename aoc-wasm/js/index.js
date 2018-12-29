import("../crate/pkg").then(module => {
  console.log("?");
  window.solve_day = module.solve_day;
});
