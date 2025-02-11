export default function NotFound() {
    return (
        <div
            className="mx-auto flex max-w-xl flex-col items-center justify-center text-center"
            style={{height: "calc(100vh - 80px)"}}
        >
            <h1 className="mt-1 text-3xl text-th-fgd-1 sm:text-4xl">Not found</h1>
            <p className="mt-2 text-lg">{404}</p>
        </div>
    );
}
