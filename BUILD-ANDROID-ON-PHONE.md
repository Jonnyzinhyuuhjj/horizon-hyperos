# Build the public Android APK from a phone

This repository includes `.github/workflows/android-apk.yml` so the Android APK can be built on GitHub-hosted runners without a local PC.

1. Create a **private** GitHub repository and upload the contents of this folder to its default branch (not the outer folder itself).
2. Open the repository's **Actions** tab. If prompted, enable GitHub Actions.
3. Select **Build Android APK** in the workflow list and tap **Run workflow**.
4. Keep `arm64-v8a` selected for most modern Android phones, including the Moto G52.
5. Wait for the run to finish. Open the successful run, scroll to **Artifacts**, and download `horizon-android-apk-arm64-v8a`.
6. Extract the downloaded artifact ZIP and install `Horizon-Android-public-arm64-v8a.apk`. Android may ask you to allow installs from the browser or file manager you used.

The workflow builds a public APK from the checked-in source and uploads it as a temporary GitHub Actions artifact (14-day retention). It does not need a signing secret; the build script creates a temporary development signing key on the runner. This APK is separate from any official release and may not update an existing installation signed with a different key.

The workflow builds only the selected ABI. Use `arm64-v8a` for the Moto G52. The app's Android manifest remains configured for `sensorLandscape`.
