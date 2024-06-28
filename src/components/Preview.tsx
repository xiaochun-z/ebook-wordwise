export default function Preview({ innerHTML }: PreviewProps) {
  return (
    <div
      dangerouslySetInnerHTML={{ __html: innerHTML }}
      style={{ maxHeight: "215px", overflow: "hidden" }}
      id="preview"
      className="overflow-ellipsis bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 p-4 font-normal text-gray-700 dark:text-gray-400 line-clamp-8"
    ></div>
  );
}

export interface PreviewProps {
  innerHTML: string;
}
