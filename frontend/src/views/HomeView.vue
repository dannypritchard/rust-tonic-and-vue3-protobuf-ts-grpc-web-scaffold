<script setup lang="ts">
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { VotingClient } from '@/protos/voting.client';
import { ref } from 'vue';
import type { Vote } from '@/protos/voting';

let transport = new GrpcWebFetchTransport({
    baseUrl: 'http://127.0.0.1:8009',
    format: 'binary',
});

const list_result = ref<Vote[] | undefined>(undefined);
const get_result = ref<Vote | undefined>(undefined);
const vote_result = ref<Vote | undefined>(undefined);

const id = ref<number | null>(1);
const url = ref<string | null>(null);

let client = new VotingClient(transport);

const list = async () => {
    list_result.value = undefined;

    try {
        let { response } = await client.index({});
        list_result.value = response.votes;
    } catch (error: any) {
        alert(error.RpcError);
    }
};

const get = async () => {
    if (!id.value) {
        alert('bugger off');
        return;
    }

    get_result.value = undefined;

    try {
        let { response } = await client.get({
            id: id.value,
        });

        get_result.value = response.vote;
    } catch (error: any) {
        alert(error.RpcError);
    } finally {
        id.value = null;
    }
};


const vote = async () => {
    if (!url.value) {
        alert('bugger off');
        return;
    }

    vote_result.value = undefined;

    try {
        let { response } = await client.vote({ url: url.value, vote: 1 });
        vote_result.value = response.vote;
    } catch (error: any) {
        alert(error.RpcError);
    } finally {
        url.value = null;
    }
};

</script>

<template>
    <div class="w-full max-w-lg m-auto">
        <h1 class="text-3xl font-bold underline">Vote</h1>

        <form @submit.prevent="list" class="border-b border-teal-500 py-2">
            <div class="flex items-center">
                <button class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded">List</button>
            </div>
            <div v-if="list_result">
                Result: <code>{{ list_result }}</code>
            </div>
        </form>

        <form @submit.prevent="get" class="border-b border-teal-500 py-2">
            <div class="flex items-center">
                <input v-model="id" type="number" min="1" minlength="1" class="appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none" />
                <button class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded">Get</button>
            </div>
            <div v-if="get_result">
                Result: <code>{{ get_result }}</code>
            </div>
        </form>

        <form @submit.prevent="vote" class="border-b border-teal-500 py-2">
            <div class="flex items-center">
                <input v-model="url" class="appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none" />
                <button class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded">Submit</button>
            </div>
            <div v-if="vote_result">
                Result: <code>{{ vote_result }}</code>
            </div>
        </form>
    </div>
</template>
