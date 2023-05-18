const spawn = require("child_process").spawn;
const gulp = require("gulp");
const babel = require("gulp-babel");
const css = require("gulp-css");
const path = require("path");
const rimraf = require("gulp-rimraf");


const cmd = name => path.join(".", "node_modules", ".bin", name);
const args = more => (Array.isArray(more) ? ["."].concat(more) : ["."]);
const exit = () => process.exit();



gulp.task('clean', function() {
    return gulp
        .src('build', { read: false, allowEmpty: true })
        .pipe(rimraf());
})

gulp.task('clean-dist', function() {
    return gulp
        .src('dist', { read: false, allowEmpty: true })
        .pipe(rimraf());
})

/* Build */

gulp.task("build-css", function() {
  return gulp
    .src("app/**/*.css")
    .pipe(css())
    .pipe(gulp.dest("build/"));
});

gulp.task("build-js", () => {
  return gulp
    .src(["app/**/*.js", "app/*.js"])
    .pipe(babel())
    .on("error", console.error.bind(console))
    .pipe(gulp.dest("build/"));
});

gulp.task("build", gulp.series("build-css", "build-js"));

/* Copy */

gulp.task("copy-html", () => {
  return gulp.src("app/public/*.html").pipe(gulp.dest("build/public"));
});

gulp.task("copy-assets", () => {
  return gulp.src("assets/**/*").pipe(gulp.dest("build/assets"));
});

gulp.task("copy", gulp.parallel("copy-html", "copy-assets"));

/* Execute */



gulp.task(
  "start",
  gulp.series("clean", "copy", "build", () => {
    spawn(cmd("electron"), args(), {
      stdio: "inherit",
      cwd: ".",
      shell: true,
      env: {...process.env, ELECTRON_ENV: 'dev'}
    }).on("close", exit);

    return new Promise(function(resolve, reject) {
      console.log(
        "Finished copying and building. Now starting your Electron App."
      );
      resolve();
    });
  })
);

gulp.task(
  "release-mac",
  gulp.series("clean", "clean-dist", "copy", "build", () => {
    spawn(cmd("electron-builder"), ["build", "--mac"], {
      stdio: "inherit",
      shell: true,
      env: {...process.env}
    }).on("close", exit);

    return new Promise(function(resolve, reject) {
      console.log("Finished building the app.");
      resolve();
    });
  })
);

gulp.task(
    "release-win",
    gulp.series("clean", "clean-dist", "copy", "build", () => {
      spawn(cmd("electron-builder"), ["build", "--win"], {
        stdio: "inherit",
        shell: true,
        env: {...process.env}
      }).on("close", exit);
  
      return new Promise(function(resolve, reject) {
        console.log("Finished building the app.");
        resolve();
      });
    })
  );

// gulp.task(
//   "test",
//   gulp.series("copy", "build", () => {
//     spawn(cmd("jest"), args(), { stdio: "inherit", cwd: ".", shell: true }).on(
//       "close",
//       exit
//     );
//   })
// );

/* Watch */
gulp.task(
  "just-copy",
  gulp.series("copy", "build", () => {
    return new Promise(function(resolve, reject) {
      console.log("Finished copying and building.");
      resolve();
    });
  })
);

gulp.task(
  "run",
  gulp.series("clean", "start", function() {
    gulp.watch(["app/**/*"], gulp.series("just-copy"));
    //NOTE: The reloading is being done from inside electron app in main.js using an electron-reload module
    //      We dont need to restart the module unless we need a hard rest and restart of the electron app itself.
    //      Which has to be done manually because of unexpected problems with the electron-reload module.
    return new Promise(function(resolve, reject) {
      resolve();
    });
  })
);