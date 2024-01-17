import Visualize from "./components/visualize/Visualize";

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-center p-4">
      <h1 className="font-extrabold text-2xl">Welcome to ReMnK</h1>
      <Visualize />
    </main>
  );
}
