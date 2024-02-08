import { useCallback, useEffect, useMemo, useState } from "react";
import type { File } from "./types";
import Image from "next/image";
import { Folder_Light, Document_Light } from "@muse/assets/icons";
import styles from "./styles.module.css";

type Props = {
  files: File[];
  goToFolder: (folderName: string) => void;
}

type FileWithId = File & { id: string };

export default function Files({ files, goToFolder }: Props) {
  let filesWithId = useMemo(() => {
    return files.map((file) => {
      return { ...file, id: Math.floor(Math.random() * 10000000).toString() };
    })
  }, [files]);

  let [selectedId, setSelectedId] = useState<string | null>(null);

  let handleDoubleClick = useCallback((file: File) => {
    if (file.is_dir) {
      goToFolder(file.name);
    } else {
      console.log("openfile", file.name);
    }
  }, [goToFolder]);

  return (
    <div className="p-6 mt-2 bg-white">
      <div className="flex flex-wrap">
        <div
          className="w-36 m-2 flex flex-col justify-between overflow-hidden cursor-pointer select-none"
          onClick={() => goToFolder("-1")}
        >
          <div className="rounded-lg w-36 h-36 flex justify-center items-center
            bg-slate-200 text-xs">返回上级</div>
        </div>
        {filesWithId.map((file) => (
          <div
            key={file.id}
            className={
              `w-36 m-2 flex flex-col justify-between overflow-hidden cursor-default select-none
              ${selectedId === file.id && styles["selected"]}`
            }
            onClick={() => setSelectedId(file.id)}
            onDoubleClick={() => handleDoubleClick(file)}
          >
            <div className={`${styles["image"]} rounded-lg`}>
              {file.is_dir ? (
                <Image src={ Folder_Light } alt="folder"></Image>
              ) : (
                <Image src={ Document_Light } alt="folder"></Image>
              )}
            </div>
            <div className={`${styles["title"]} p-1 mt-1 mb-2 rounded-lg`}>
              <div className="leading-[1.4em] h-[2.8em] line-clamp-2 text-xs text-center">{file.name}</div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
