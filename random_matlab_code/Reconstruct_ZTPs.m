function ZTP_node = Reconstruct_ZTPs(v, numerators, denominator, d_tilde, N)


% This function is to reconstruct all zero-terminated paths (ZTPs) from the
% irreducible error events (IEEs). Traditional method by Lou et al. does
% not adapt to the high-rate ZTCCs due to nontrivial terminations.
%
% Input parameters:
%   1) v: (v-1) denotes # memory elements in systematic feedback encoder.
%   2) numerators: a 1-by-k row vector with entries in octal, where k
%   denotes the # input rails
%   3) denominator: a scalar in octal
%   4) d_tilde: a scalar denoting the distance threshold (achievable)
%   5) N: a scalar denoting the primal trellis length
%
% Output parameters: ZTP_node a struct composed of following fields
%   1) list: a d_tilde-by-1 column vector denoting the list of length-kN
%       ZTPs arranged in ascending distances. The true distance is one less
%       than its index.
%   2) aggregate: a scalar denoting the number of length-kN ZTPs of
%       distance less than 'd_tilde'.
%
% Remarks:
%   1) Need to run "find_irreducible_error_event.m" if IEEs are not
%       generated before.
%   2) Need to run "Compute_ZTCC_weight_spectrum.m" if weight_node is not
%       generated before.
%   3) The distance index is true distance plus one
%
% Written by Hengjie Yang (hengjie.yang@ucla.edu)   04/17/21
%


% Step 1: load files
ZTP_node = {};

path = './Simulation_results/';
k = length(numerators);
num_string = '';
for iter = 1:k
    num_string = [num_string, num2str(numerators(iter)), '_'];
end

fileName = ['error_events_v_',num2str(v),'_num_',num_string,'den_',...
    num2str(denominator),'_d_tilde_',num2str(d_tilde)];

if ~exist([path, fileName, '.mat'], 'file')
    disp(['Error: the file ',fileName, ' does not exist!']);
    return
end

load([path, fileName, '.mat'], 'error_events', 'error_event_lengths');

fileName = ['weight_spectrum_v_',num2str(v), '_num_', num_string,'den_',...
    num2str(denominator),'_N_',num2str(N)];

if ~exist([path, fileName, '.mat'], 'file')
    disp(['Error: the file ',fileName, ' does not exist!']);
    return
end

load([path, fileName, '.mat'], 'weight_node');


full_weight_spectrum = weight_node.weight_spectrum;
d_max = size(full_weight_spectrum, 1);
if d_tilde > d_max - 1
    msg = 'Error: d_tilde exceeds the maximum possible distance!';
    disp(msg);
    return
end

 



% Step 2: use dynamic programming to reconstruct the length-kN ZTPs.
disp('Step 2: use dynamic programming to reconstruct ZTPs with objective length.');
ZTPs = cell(d_tilde+1, 1);

% preprocessing: need to add zeros(1,k) to IEEs
IEEs = cell(d_tilde+1, 1);
IEE_lens = cell(d_tilde+1, 1);
IEEs{1} = zeros(1, k);
IEE_lens{1} = k;
for dist = 1:d_tilde % true distance
    IEEs{dist+1} = error_events{dist};
    IEE_lens{dist+1} = error_event_lengths{dist};
end


% Suppress 'already exists' 'no file exists' warnings
warning('off','all')
[status, err] = mkdir('./tmp_ZTPs');
if status == 0
    disp(err);
    disp("Error: An error occured when attempting to make ZTP partition files");
    return;
end

ZTP = [];
for dist = 0:d_tilde
    dir_name = sprintf('./tmp_ZTPs/dist_%d',dist + 1);
    [status, err] = mkdir(dir_name);
    if status == 0
        disp(err);
        disp("Error: An error occured when attempting to make ZTP partition files");
        return;
    end
    for len = 1:N
        file_name = sprintf("%s/dist_%d_len_%d.mat", dir_name, dist + 1, len + 1);
        save(file_name, "ZTP");
    end
end

warning('on','all')

Temp_ZTPs = cell(d_tilde+1, N+1);

% for dist = 0:d_tilde % enumerate true objective distance
%     disp(['     Current distance: ',num2str(dist)]);
%     for len = 1:N % enumerate true objective trellis length
%         for weight = dist:-1:0 % enumerate true IEE weight
%             for ii = 1:size(IEEs{weight+1}, 1)
%                 l = IEE_lens{weight+1}(ii) / k; % convert to trellis length
%                 if weight == dist && l == len
%                     Temp_ZTPs{dist+1, len+1} = int8(Temp_ZTPs{dist+1, len+1}); % save memory
%                     Temp_ZTPs{dist+1, len+1} = [Temp_ZTPs{dist+1, len+1}; IEEs{weight+1}(ii, 1:(k*l))];
%                 elseif l < len && ~isempty(Temp_ZTPs{dist-weight+1, len-l+1})
%                     [row, ~] = size(Temp_ZTPs{dist-weight+1, len-l+1});
%                     Added_bits = repmat(IEEs{weight+1}(ii, 1:(k*l)), row, 1);
%                     New_ZTPs = [Temp_ZTPs{dist-weight+1, len-l+1}, Added_bits];
%                     Temp_ZTPs{dist+1, len+1} = int8(Temp_ZTPs{dist+1, len+1}); % save memory
%                     Temp_ZTPs{dist+1, len+1} = [Temp_ZTPs{dist+1, len+1}; New_ZTPs];
%                 end
%             end
%         end
%     end
% end

for dist = 0:d_tilde % enumerate true objective distance
    disp(['     Current distance: ',num2str(dist)]);
    for len = 1:N % enumerate true objective trellis length
        file = Get_file(dist, len);
        ZTP_str_1 = matfile(sprintf("%s/%s", file.folder, file.name));
        Temp_ZTP_1 = ZTP_str_1.ZTP;

        for weight = dist:-1:0 % enumerate true IEE weight

            for ii = 1:size(IEEs{weight+1}, 1)
                l = IEE_lens{weight+1}(ii) / k; % convert to trellis length

                file2 = Get_file(dist - weight, len - l);
                if ~isempty(file2)
                    file2_path_txt = sprintf("%s/%s", file2.folder, file2.name);
                    ZTP_str_2 = matfile(file2_path_txt);
                    Temp_ZTP_2 = ZTP_str_2.ZTP;
                else
                    Temp_ZTP_2 = [];
                end
                
                if weight == dist && l == len
                   % TODO: put the matrices within each distance and length into memory
                    
                   Temp_ZTP_1 = int8(Temp_ZTP_1); % save memory
                   Temp_ZTP_1 = [Temp_ZTP_1; IEEs{weight+1}(ii, 1:(k*l))];
                elseif l < len && ~isempty(Temp_ZTP_2)
                    [row, ~] = size(Temp_ZTP_2);
                    Added_bits = repmat(IEEs{weight+1}(ii, 1:(k*l)), row, 1);
                    New_ZTPs = [Temp_ZTP_2, Added_bits];
                    Temp_ZTP_1 = int8(Temp_ZTP_1); % save memory
                    Temp_ZTP_1 = [Temp_ZTP_1; New_ZTPs];
                end
            end
        end
        % TODO: put matrix size in filename to improve performance
        % MAYBE: Just don't put the size in the file name and take the
        % performance hit?
        file_name = sprintf("./tmp_ZTPs/dist_%d/dist_%d_len_%d", dist + 1, dist + 1, len + 1);
        ZTP = [];
        if ~isempty(Temp_ZTP_1)
            ZTP = Temp_ZTP_1;
        end
        save(file_name, "ZTP");
    end
end


% After building, extract the objective length ZTPs
for dist = 0:d_tilde
    file = Get_file(dist, len);
    if ~isempty(file)
        ZTP_str = matfile(sprintf("%s/%s", file.folder, file.name));
        %ZTP_str = fileread(sprintf("./tmp_ZTPs/dist_%d/dist_%d_len_%d.json", dist + 1, dist + 1, N + 1));
        Temp_ZTP = ZTP_str.ZTP;
        if ~isempty(Temp_ZTP)
            ZTPs{dist+1} = Temp_ZTP;
        end
    end
end

[stat, mess, id] = rmdir('tmp_ZTPs', 's');

% for dist = 0:d_tilde
%     if ~isempty(Temp_ZTPs{dist+1, N+1})
%         ZTPs{dist+1} = Temp_ZTPs{dist+1, N+1};
%     end
% end

clearvars Temp_ZTPs


% Step 3: check if shifting is required.
need_shift_flag = 0;

for dist = 0:d_tilde
    if size(ZTPs{dist+1}, 1) ~= full_weight_spectrum(dist+1)
        need_shift_flag = 1;
        break
    end
end


aggregate = 0;
for dist = 0:d_tilde
    aggregate = aggregate + size(ZTPs{dist+1}, 1);
end

ZTP_node.list = ZTPs;
ZTP_node.aggregate = aggregate;


if need_shift_flag == 0
    disp('Congratulations! No need to shift before saving results!');
else
    disp('Sorry, the shifting operation is required...');
end

fileName = ['ZTP_node_v_',num2str(v), '_num_', num_string,'den_',...
    num2str(denominator), '_d_',num2str(d_tilde),'_N_',num2str(N)];
save([path, fileName],'ZTP_node','-v7.3');



end


function file = Get_file(dist, len)
    file = char.empty;
    file_list = dir(sprintf("./tmp_ZTPs/dist_%d/dist_%d_len_%d*", dist + 1, dist+1, len+1));
    if isempty(file_list)
        return;
    end
    file = file_list(1);
end


function boolean = Check_empty(file_name)
% Checks if the matrix stored within json file 'file_name' is
% empty or not
    boolean = 0;
    expr = "0x0";
    if regexp(file_name, expr) ~= []
        boolean = 0;
    else
        boolean = 1;
    end
end