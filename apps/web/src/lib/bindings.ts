// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "assets.list", input: FilePathQueryPayload, result: FilePathQueryResult[] } | 
        { key: "files.home_dir", input: never, result: string } | 
        { key: "files.ls", input: string, result: any } | 
        { key: "libraries.list", input: never, result: any } | 
        { key: "users.list", input: never, result: any } | 
        { key: "version", input: never, result: string } | 
        { key: "video.search.all", input: string, result: SearchResultPayload[] } | 
        { key: "video.tasks.list", input: never, result: VideoTaskResult[] },
    mutations: 
        { key: "assets.create_asset_object", input: AssetObjectCreatePayload, result: string } | 
        { key: "assets.create_file_path", input: FilePathCreatePayload, result: string } | 
        { key: "files.reveal", input: string, result: null } | 
        { key: "libraries.create", input: string, result: any } | 
        { key: "video.tasks.create", input: string, result: any },
    subscriptions: never
};

export type AssetObjectCreatePayload = { path: string; localFullPath: string }

export type AssetObjectQueryResult = { id: number; localFullPath: string }

export type FilePathCreatePayload = { path: string; name: string }

export type FilePathQueryPayload = { path: string; dirsOnly: boolean }

export type FilePathQueryResult = { id: number; name: string; isDir: boolean; assetObject: AssetObjectQueryResult | null }

export type SearchResultPayload = { imagePath: string; videoPath: string; startTime: number }

export type VideoTaskResult = { id: number; videoPath: string; videoFileHash: string; taskType: string; startsAt: string | null; endsAt: string | null }
