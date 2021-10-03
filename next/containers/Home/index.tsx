import React from 'react';
import TextField from '@mui/material/TextField';
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';


const Home = () => (
    <div className='home-container'>
        <Box
            sx={{
                padding: '10px',
            }}
        >
            <Grid item xs={12}>
                <TextField
                    id="outlined-multiline-static"
                    multiline
                    fullWidth
                    rows={4}
                    defaultValue="Input"
                    variant="filled"
                />
            </Grid>
            <Grid item xs={12}>
                <Grid xs={4}>
                    <Button>翻译</Button>
                </Grid>
            </Grid>
            <Grid item xs={12}>
                <div><TextField
                    id="outlined-multiline-static"
                    multiline
                    fullWidth
                    rows={4}
                    defaultValue="Output"
                    variant="filled"
                /></div>
            </Grid>
        </Box>
    </div>
)

export default Home;
